import { FastifyInstance, FastifyRequest, FastifyReply } from 'fastify';
import { requireAuth, getUserName } from '../middleware/auth.js';
import {
  titleDir,
  ensureDir,
  readManifest,
  writeManifest,
  GameEntry,
} from '../storage/disk.js';
import fs from 'fs';
import path from 'path';
import crypto from 'crypto';

interface SaveParams {
  titleId: string;
}

function computeSha256(filePath: string): string {
  const hash = crypto.createHash('sha256');
  hash.update(fs.readFileSync(filePath));
  return 'sha256:' + hash.digest('hex');
}

export async function savesRoutes(app: FastifyInstance): Promise<void> {
  app.put<{ Params: SaveParams }>(
    '/api/save/:titleId',
    { preHandler: requireAuth },
    async (request: FastifyRequest<{ Params: SaveParams }>, reply: FastifyReply) => {
      const { titleId } = request.params;
      const deviceId = (request.headers['x-device-id'] as string) ?? 'unknown';
      const clientHash = request.headers['x-save-hash'] as string | undefined;
      const timestamp =
        (request.headers['x-save-timestamp'] as string) ??
        new Date().toISOString();

      const userName = getUserName();
      const dir = titleDir(userName, titleId);
      ensureDir(dir);

      const tmpPath = path.join(dir, `upload_${Date.now()}.tmp`);
      const currentPath = path.join(dir, 'current.zip');
      const versionsDir = path.join(dir, 'versions');

      const body = await request.file();
      if (!body) {
        return reply.code(400).send({ ok: false, error: 'No file in request' });
      }

      // stream to temp file via toBuffer (more reliable with Fastify multipart)
      const buffer = await body.toBuffer();
      fs.writeFileSync(tmpPath, buffer);

      // verify hash if provided
      const actualHash = computeSha256(tmpPath);
      if (clientHash && clientHash !== actualHash) {
        fs.unlinkSync(tmpPath);
        return reply
          .code(400)
          .send({ ok: false, error: `Hash mismatch: expected ${clientHash}, got ${actualHash}` });
      }

      // rotate current → versions/
      if (fs.existsSync(currentPath)) {
        ensureDir(versionsDir);
        const versionName = timestamp.replace(/[:.]/g, '-') + '.zip';
        fs.renameSync(currentPath, path.join(versionsDir, versionName));
      }

      fs.renameSync(tmpPath, currentPath);
      const size = fs.statSync(currentPath).size;

      // update manifest
      const manifest = readManifest(userName);
      const entry: GameEntry = {
        ...(manifest.games[titleId] ?? {}),
        latestVersion: timestamp,
        latestHash: actualHash,
        uploadedBy: deviceId,
        size,
      };
      manifest.games[titleId] = entry;
      manifest.updatedAt = new Date().toISOString();
      writeManifest(userName, manifest);

      reply.send({ ok: true, titleId, version: timestamp, hash: actualHash });
    }
  );

  app.get<{ Params: SaveParams }>(
    '/api/save/:titleId',
    { preHandler: requireAuth },
    async (request: FastifyRequest<{ Params: SaveParams }>, reply: FastifyReply) => {
      const { titleId } = request.params;
      const userName = getUserName();
      const currentPath = path.join(titleDir(userName, titleId), 'current.zip');

      if (!fs.existsSync(currentPath)) {
        return reply.code(404).send({ ok: false, error: 'No save found for ' + titleId });
      }

      const manifest = readManifest(userName);
      const entry = manifest.games[titleId];

      const data = fs.readFileSync(currentPath);
      reply
        .header('Content-Type', 'application/zip')
        .header('Content-Disposition', `attachment; filename="${titleId}.zip"`)
        .header('X-Save-Hash', entry?.latestHash ?? '')
        .header('X-Save-Timestamp', entry?.latestVersion ?? '')
        .send(data);
    }
  );

  app.delete<{ Params: SaveParams }>(
    '/api/save/:titleId',
    { preHandler: requireAuth },
    async (request: FastifyRequest<{ Params: SaveParams }>, reply: FastifyReply) => {
      const { titleId } = request.params;
      const userName = getUserName();
      const dir = titleDir(userName, titleId);

      if (!fs.existsSync(dir)) {
        return reply.code(404).send({ ok: false, error: 'No save found for ' + titleId });
      }

      fs.rmSync(dir, { recursive: true, force: true });

      const manifest = readManifest(userName);
      delete manifest.games[titleId];
      manifest.updatedAt = new Date().toISOString();
      writeManifest(userName, manifest);

      reply.send({ ok: true, titleId });
    }
  );
}
