import { browser, defineBackground } from '#imports';
import { api } from '../lib/api';

export default defineBackground(() => {
  let tabLimit = 10;
  let isServerOnline = false;

  async function refreshTabLimit() {
    try {
      tabLimit = await api.getTabLimit();
      isServerOnline = true;
    } catch {
      isServerOnline = false;
    }
  }

  async function updateBadge() {
    const tabs = await browser.tabs.query({});
    const count = tabs.length;

    browser.action.setBadgeText({ text: String(count) });

    if (!isServerOnline) {
      browser.action.setBadgeBackgroundColor({ color: '#a1a1aa' });
    } else if (count > tabLimit) {
      browser.action.setBadgeBackgroundColor({ color: '#ef4444' });
    } else if (count > tabLimit - 2) {
      browser.action.setBadgeBackgroundColor({ color: '#f59e0b' });
    } else {
      browser.action.setBadgeBackgroundColor({ color: '#22c55e' });
    }
  }

  browser.tabs.onCreated.addListener(updateBadge);
  browser.tabs.onRemoved.addListener(updateBadge);

  // Refresh on startup and periodically
  refreshTabLimit().then(updateBadge);

  // Poll server every 60 seconds for tab limit changes
  setInterval(() => {
    refreshTabLimit().then(updateBadge);
  }, 60_000);
});
