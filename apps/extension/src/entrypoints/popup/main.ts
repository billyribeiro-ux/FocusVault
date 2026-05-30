import { browser } from '#imports';
import { api } from '../../lib/api';

const $title = document.getElementById('title') as HTMLInputElement;
const $url = document.getElementById('url') as HTMLInputElement;
const $tags = document.getElementById('tags') as HTMLInputElement;
const $type = document.getElementById('type') as HTMLSelectElement;
const $captureBtn = document.getElementById('capture-btn') as HTMLButtonElement;
const $feedback = document.getElementById('feedback') as HTMLParagraphElement;
const $statusDot = document.getElementById('status-dot') as HTMLSpanElement;
const $tabCurrent = document.getElementById('tab-current') as HTMLSpanElement;
const $tabLimit = document.getElementById('tab-limit') as HTMLSpanElement;
const $offlineMsg = document.getElementById('offline-msg') as HTMLDivElement;
const $captureSection = document.querySelector('.capture-section') as HTMLDivElement;
const $tabCounter = document.getElementById('tab-counter') as HTMLDivElement;

// ── Initialize ──

async function init() {
  // Fill in current tab info
  const [tab] = await browser.tabs.query({ active: true, currentWindow: true });
  if (tab) {
    $title.value = tab.title || '';
    $url.value = tab.url || '';
  }

  // Check server health
  const online = await api.healthCheck();
  if (online) {
    $statusDot.className = 'status-dot status-online';
    $statusDot.title = 'Server connected';
    $offlineMsg.style.display = 'none';
    $captureSection.style.display = '';
    $tabCounter.style.display = '';

    // Load tab count + limit
    await updateTabCount();
  } else {
    $statusDot.className = 'status-dot status-offline';
    $statusDot.title = 'Server offline';
    $offlineMsg.style.display = '';
    $captureSection.style.display = 'none';
    $tabCounter.style.display = 'none';
  }
}

async function updateTabCount() {
  try {
    const tabs = await browser.tabs.query({});
    const count = tabs.length;
    $tabCurrent.textContent = String(count);

    const limit = await api.getTabLimit();
    $tabLimit.textContent = String(limit);

    if (count > limit) {
      $tabCurrent.classList.add('tab-over');
    } else {
      $tabCurrent.classList.remove('tab-over');
    }
  } catch {
    $tabCurrent.textContent = '-';
    $tabLimit.textContent = '-';
  }
}

// ── Capture ──

$captureBtn.addEventListener('click', async () => {
  const title = $title.value.trim();
  if (!title) {
    showFeedback('Title is required', 'error');
    return;
  }

  $captureBtn.disabled = true;
  $captureBtn.textContent = 'Capturing...';

  try {
    const tagsRaw = $tags.value.trim();
    const tags = tagsRaw ? tagsRaw.split(',').map((t) => t.trim()).filter(Boolean) : [];

    await api.captureToVault({
      item_type: $type.value,
      title,
      url: $url.value || undefined,
      tags,
      capture_source: 'extension',
    });

    showFeedback('Captured!', 'success');
    setTimeout(() => window.close(), 800);
  } catch (err) {
    showFeedback(err instanceof Error ? err.message : 'Capture failed', 'error');
  } finally {
    $captureBtn.disabled = false;
    $captureBtn.textContent = 'Capture to Vault';
  }
});

function showFeedback(msg: string, type: 'success' | 'error') {
  $feedback.textContent = msg;
  $feedback.className = `feedback ${type}`;
}

init();
