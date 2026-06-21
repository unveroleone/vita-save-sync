import { FastifyInstance } from 'fastify';
import { requireAuth, getToken, getUserName } from '../middleware/auth.js';
import {
  devicesDir,
  ensureDir,
  writeManifest,
  readManifest,
} from '../storage/disk.js';
import fs from 'fs';
import path from 'path';

interface PairBody {
  token: string;
  deviceName: string;
}

export async function authRoutes(app: FastifyInstance): Promise<void> {
  app.post<{ Body: PairBody }>('/api/pair', async (request, reply) => {
    const { token, deviceName } = request.body;
    if (token !== getToken()) {
      return reply.code(401).send({ ok: false, error: 'Invalid token' });
    }

    const userName = getUserName();
    const dir = devicesDir(userName);
    ensureDir(dir);

    const deviceRecord = {
      deviceId: deviceName,
      pairedAt: new Date().toISOString(),
    };
    fs.writeFileSync(
      path.join(dir, `${deviceName}.json`),
      JSON.stringify(deviceRecord, null, 2)
    );

    // ensure manifest exists
    const manifest = readManifest(userName);
    writeManifest(userName, manifest);

    reply.send({ userId: userName, deviceId: deviceName, ok: true });
  });
}
