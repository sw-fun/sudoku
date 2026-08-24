// Service worker for the Sudoku PWA, scoped to the /sudoku/ Pages
// sub-path. Register with a relative URL ('sw.js') so the same file
// works under /sudoku/ in production and at the root during local
// development.
const CACHE_NAME = 'sudoku-v0.6.3';
const BASE = new URL('sw.js', self.registration.scope).pathname.replace(/\/sw\.js$/, '/');
const urlsToCache = [
  BASE,
  BASE + 'index.html',
  BASE + 'manifest.json',
  BASE + 'favicon.ico'
];

self.addEventListener('install', event => {
  event.waitUntil(
    caches.open(CACHE_NAME)
      .then(cache => cache.addAll(urlsToCache))
      .then(() => self.skipWaiting())
  );
});

self.addEventListener('fetch', event => {
  if (event.request.method !== 'GET') {
    return;
  }
  // Cache-first for same-origin GETs; everything else passes through.
  if (new URL(event.request.url).origin !== self.location.origin) {
    return;
  }
  event.respondWith(
    caches.match(event.request).then(cached => {
      if (cached) {
        return cached;
      }
      // Network-first for navigation requests so deploys show up; fall
      // back to the cached shell when offline.
      if (event.request.mode === 'navigate') {
        return fetch(event.request)
          .then(response => {
            const copy = response.clone();
            caches.open(CACHE_NAME).then(cache => cache.put(event.request, copy));
            return response;
          })
          .catch(() => caches.match(BASE + 'index.html'));
      }
      return fetch(event.request).then(response => {
        if (response.ok) {
          const copy = response.clone();
          caches.open(CACHE_NAME).then(cache => cache.put(event.request, copy));
        }
        return response;
      });
    })
  );
});

self.addEventListener('activate', event => {
  const cacheWhitelist = [CACHE_NAME];
  event.waitUntil(
    caches.keys().then(cacheNames =>
      Promise.all(
        cacheNames
          .filter(cacheName => !cacheWhitelist.includes(cacheName))
          .map(cacheName => caches.delete(cacheName))
      )
    ).then(() => self.clients.claim())
  );
});