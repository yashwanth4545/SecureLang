import { Request, Response, NextFunction } from 'express';

// Simple logging middleware stub replacing Morgan/Winston for now
export const requestLogger = (req: Request, res: Response, next: NextFunction) => {
  const start = Date.now();
  res.on('finish', () => {
    const duration = Date.now() - start;
    console.log(`[Audit Log] ${req.method} ${req.originalUrl} - ${res.statusCode} [${duration}ms] - IP: ${req.ip}`);
  });
  next();
};
