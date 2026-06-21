import { FastifyInstance } from 'fastify';

export async function statusRoutes(app: FastifyInstance): Promise<void> {
  app.get('/api/status', async (_req, reply) => {
    reply.send({
      ok: true,
      serverVersion: '0.1.0',
      features: ['manifest', 'upload', 'download', 'history'],
    });
  });
}
