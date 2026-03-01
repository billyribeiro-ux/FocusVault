import { defineBackground } from 'wxt/sandbox';
import { api } from '../lib/api';

export default defineBackground(() => {
  let tabLimit = 10;
  let isServerOnline = false;

  // Refresh tab limit from server periodically
  async function refreshTabLimit() {
    try {
      tabLimit = await api.getTabLimit();
      isServerOnline = true;
    } catch {
      isServerOnline = false;
    }
  }

  // Update badge with current tab count
  async function updateBadge() {
    const tabs = await chrome.tabs.query({});
    const count = tabs.length;

    chrome.action.setBadgeText({ text: String(count) });

    if (!isServerOnline) {
      chrome.action.setBadgeBackgroundColor({ color: '#a1a1aa' });
    } else if (count > tabLimit) {
      chrome.action.setBadgeBackgroundColor({ color: '#ef4444' });
    } else if (count > tabLimit - 2) {
      chrome.action.setBadgeBackgroundColor({ color: '#f59e0b' });
    } else {
      chrome.action.setBadgeBackgroundColor({ color: '#22c55e' });
    }
  }

  // Listen for tab changes
  chrome.tabs.onCreated.addListener(updateBadge);
  chrome.tabs.onRemoved.addListener(updateBadge);

  // Refresh on startup and periodically
  refreshTabLimit().then(updateBadge);

  // Poll server every 60 seconds for tab limit changes
  setInterval(() => {
    refreshTabLimit().then(updateBadge);
  }, 60_000);
});
