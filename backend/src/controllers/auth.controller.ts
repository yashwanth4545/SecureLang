import { Request, Response, NextFunction } from 'express';
import argon2 from 'argon2';
import jwt from 'jsonwebtoken';
import { z } from 'zod';
import { prisma } from '../index';

const lockouts = new Map<string, { attempts: number, lockedUntil: number }>();

const registerSchema = z.object({
  email: z.string().email(),
  username: z.string().min(3).max(30),
  password: z.string().min(8)
});

const loginSchema = z.object({
  email: z.string().email(),
  password: z.string()
});

export const register = async (req: Request, res: Response, next: NextFunction): Promise<void> => {
  try {
    const { email, username, password } = registerSchema.parse(req.body);

    const existingUser = await prisma.user.findFirst({
      where: { OR: [{ email }, { username }] }
    });

    if (existingUser) {
      res.status(409).json({ success: false, message: 'User already exists' });
      return;
    }

    const passwordHash = await argon2.hash(password);

    const user = await prisma.user.create({
      data: {
        email,
        username,
        passwordHash,
      },
      select: { id: true, email: true, username: true, role: true, createdAt: true }
    });

    res.status(201).json({ success: true, user });
  } catch (error) {
    next(error);
  }
};

export const login = async (req: Request, res: Response, next: NextFunction): Promise<void> => {
  try {
    const { email, password } = loginSchema.parse(req.body);
    const ipAddress = req.ip || req.socket.remoteAddress || 'unknown';
    const userAgent = req.headers['user-agent'] || 'unknown';

    // Lockout check
    const lockoutState = lockouts.get(email);
    if (lockoutState && Date.now() < lockoutState.lockedUntil) {
      res.status(429).json({ success: false, message: 'Account locked due to too many failed attempts. Try again later.' });
      return;
    }

    const user = await prisma.user.findUnique({ where: { email } });

    if (!user) {
      res.status(401).json({ success: false, message: 'Invalid credentials' });
      return;
    }

    const isPasswordValid = await argon2.verify(user.passwordHash, password);

    if (!isPasswordValid) {
      const attempts = (lockoutState?.attempts || 0) + 1;
      if (attempts >= 5) {
        lockouts.set(email, { attempts, lockedUntil: Date.now() + 15 * 60 * 1000 });
      } else {
        lockouts.set(email, { attempts, lockedUntil: 0 });
      }
      
      await prisma.loginLog.create({
        data: { userId: user.id, ipAddress, userAgent, status: 'FAILED' }
      });
      res.status(401).json({ success: false, message: 'Invalid credentials' });
      return;
    }

    lockouts.delete(email);

    const jwtSecret = process.env.JWT_SECRET;
    if (!jwtSecret) {
      res.status(500).json({ success: false, message: 'Internal Server Error' });
      return;
    }

    const token = jwt.sign(
      { userId: user.id, role: user.role },
      jwtSecret,
      { expiresIn: '1h' }
    );

    const expiresAt = new Date();
    expiresAt.setHours(expiresAt.getHours() + 1);

    await prisma.session.create({
      data: {
        userId: user.id,
        token,
        expiresAt
      }
    });

    await prisma.loginLog.create({
      data: { userId: user.id, ipAddress, userAgent, status: 'SUCCESS' }
    });

    res.status(200).json({ success: true, token, user: { id: user.id, email: user.email, username: user.username, role: user.role } });
  } catch (error) {
    next(error);
  }
};

export const logout = async (req: Request, res: Response, next: NextFunction): Promise<void> => {
  try {
    const token = req.header('Authorization')?.replace('Bearer ', '');
    
    if (token) {
      await prisma.session.deleteMany({ where: { token } });
    }

    res.status(200).json({ success: true, message: 'Logged out successfully' });
  } catch (error) {
    next(error);
  }
};
