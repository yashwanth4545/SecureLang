import { Router, Request, Response } from 'express';
import { authenticate, requireRole } from '../middlewares/auth.middleware';
import { prisma } from '../index';

const router = Router();

interface AuthRequest extends Request {
  user?: any;
}

router.get('/profile', authenticate, (req: AuthRequest, res: Response) => {
  res.status(200).json({ success: true, user: req.user });
});

router.get('/security', authenticate, async (req: AuthRequest, res: Response) => {
  try {
    const logs = await prisma.loginLog.findMany({
      where: { userId: req.user.id },
      orderBy: { createdAt: 'desc' },
      take: 10
    });
    res.status(200).json({ success: true, logs });
  } catch (error) {
    res.status(500).json({ success: false, message: 'Internal Server Error' });
  }
});

export default router;
