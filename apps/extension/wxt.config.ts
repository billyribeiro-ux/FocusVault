import { defineConfig } from 'wxt';

export default defineConfig({
  srcDir: 'src',
  manifest: {
    name: 'FocusVault',
    description: 'Quick-capture tabs and links to your FocusVault',
    permissions: ['tabs', 'activeTab', 'storage'],
    host_permissions: ['http://localhost:3000/*', 'http://127.0.0.1:*/*'],
  },
});
