import { FastifyInstance } from 'fastify';
import { requireAuth, getUserName } from '../middleware/auth.js';
import { devicesDir } from '../storage/disk.js';
import fs from 'fs';
import path from 'path';

export async function devicesRoutes(app: FastifyInstance): Promise<void> {
  app.get('/api/devices', { preHandler: requireAuth }, async (_req, reply) => {
    const userName = getUserName();
    const dir = devicesDir(userName);

    if (!fs.existsSync(dir)) {
      return reply.send([]);
    }

    const devices = fs
      .readdirSync(dir)
      .filter(f => f.endsWith('.json'))
      .map(f => {
        try {
          return JSON.parse(fs.readFileSync(path.join(dir, f), 'utf8'));
        } catch {
          return null;
        }
      })
      .filter(Boolean);

    reply.send(devices);
  });
}
