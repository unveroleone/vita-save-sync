import Fastify from 'fastify';
import multipart from '@fastify/multipart';
import { statusRoutes } from './routes/status.js';
import { authRoutes } from './routes/auth.js';
import { manifestRoutes } from './routes/manifest.js';
import { savesRoutes } from './routes/saves.js';

const app = Fastify({ logger: true });

app.register(multipart, {
  limits: {
    fileSize: 256 * 1024 * 1024, // 256 MB per save zip
  },
});

app.register(statusRoutes);
app.register(authRoutes);
app.register(manifestRoutes);
app.register(savesRoutes);

const port = parseInt(process.env.PORT ?? '3000', 10);

app.listen({ port, host: '0.0.0.0' }, (err) => {
  if (err) {
    app.log.error(err);
    process.exit(1);
  }
  app.log.info(`Save Sync server listening on port ${port}`);
});
