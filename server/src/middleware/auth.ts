import { FastifyRequest, FastifyReply } from 'fastify';

export function getToken(): string {
  const token = process.env.USER_TOKEN;
  if (!token) throw new Error('USER_TOKEN env var is not set');
  return token;
}

export function getUserName(): string {
  return process.env.USER_NAME ?? 'default';
}

export async function requireAuth(
  request: FastifyRequest,
  reply: FastifyReply
): Promise<void> {
  const auth = request.headers['authorization'];
  if (!auth || !auth.startsWith('Bearer ')) {
    reply.code(401).send({ ok: false, error: 'Missing bearer token' });
    return;
  }
  const token = auth.slice(7);
  if (token !== getToken()) {
    reply.code(401).send({ ok: false, error: 'Invalid token' });
  }
}
