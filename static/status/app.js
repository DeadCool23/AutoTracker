(() => {
  const STATUS_URL = '/status-text';
  const pollSelect = document.getElementById('poll-interval');
  const refreshBtn = document.getElementById('refresh-btn');
  const lastUpdated = document.getElementById('last-updated');

  const elActive = document.getElementById('value-active');
  const elAHR = document.getElementById('value-ahr');
  const elRWW = document.getElementById('value-rww');

  const canvas = document.getElementById('spark');
  const ctx = canvas.getContext('2d');

  let pollTimer = null;
  let history = [];
  const maxPoints = 50;
  let lastRequestsTotal = null;
  const defaultInterval = parseInt(pollSelect.value, 10) || 5000;

  function parseStubStatus(text){
    const result = {
      active: null,
      accepts: null,
      handled: null,
      requests: null,
      reading: null,
      writing: null,
      waiting: null
    };

    const lines = text.split('\n').map(s => s.trim()).filter(Boolean);
    for (let line of lines){
      if (line.toLowerCase().startsWith('active connections:')){
        const m = line.match(/Active connections:\s*(\d+)/i);
        if(m) result.active = parseInt(m[1],10);
      } else if (/^\d+\s+\d+\s+\d+/.test(line)){
        const parts = line.split(/\s+/);
        result.accepts = parseInt(parts[0],10);
        result.handled = parseInt(parts[1],10);
        result.requests = parseInt(parts[2],10);
      } else if (/reading:/i.test(line) && /writing:/i.test(line) && /waiting:/i.test(line)){
        const m = line.match(/Reading:\s*(\d+)\s*Writing:\s*(\d+)\s*Waiting:\s*(\d+)/i);
        if(m){
          result.reading = parseInt(m[1],10);
          result.writing = parseInt(m[2],10);
          result.waiting = parseInt(m[3],10);
        }
      }
    }
    return result;
  }

  function updateUI(data){
    elActive.textContent = (data.active !== null ? data.active : '—');
    elAHR.textContent = [
      data.accepts !== null ? data.accepts : '—',
      data.handled !== null ? data.handled : '—',
      data.requests !== null ? data.requests : '—'
    ].join(' / ');
    elRWW.textContent = [
      data.reading !== null ? data.reading : '—',
      data.writing !== null ? data.writing : '—',
      data.waiting !== null ? data.waiting : '—'
    ].join(' · ');

    if (data.active !== null){
      elActive.classList.toggle('critical', data.active > 500);
      elActive.classList.toggle('ok', data.active <= 100);
    }

    if (data.requests !== null){
      if (lastRequestsTotal !== null){
        const delta = data.requests - lastRequestsTotal;
        const sec = (parseInt(pollSelect.value,10) || 5000)/1000 || 5;
        const rps = Math.max(0, Math.round(delta / sec * 100)/100);
        pushHistory(rps);
      }
      lastRequestsTotal = data.requests;
    }

    lastUpdated.textContent = 'Последнее обновление: ' + new Date().toLocaleString();
    drawSpark();
  }

  function pushHistory(val){
    if (typeof val !== 'number' || !isFinite(val)) return;
    history.push(val);
    if (history.length > maxPoints) history.shift();
  }

  function drawSpark(){
    const w = canvas.clientWidth;
    const h = canvas.clientHeight;
    canvas.width = w;
    canvas.height = h;
    const pad = 6;
    ctx.clearRect(0,0,w,h);

    if (history.length === 0) {
        ctx.fillStyle = 'rgba(255,255,255,0.03)';
        ctx.fillRect(0,0,w,h);
        ctx.fillStyle = 'rgba(255,255,255,0.15)';
        ctx.font = '12px Arial';
        ctx.fillText('Нет данных', w/2 - 28, h/2 +4);
        return;
    }

    const max = Math.max(...history);
    const min = Math.min(...history);
    const range = (max - min) || 1;

    const grad = ctx.createLinearGradient(0,0,0,h);
    grad.addColorStop(0, 'rgba(96,165,250,0.08)');
    grad.addColorStop(1, 'rgba(52,211,153,0.02)');
    ctx.fillStyle = grad;
    ctx.fillRect(0,0,w,h);

    ctx.beginPath();
    for (let i=0;i<history.length;i++){
        const x = pad + i * ((w - pad*2) / Math.max(1, history.length-1));
        const y = h - pad - ((history[i]-min)/range) * (h - pad*2);
        if(i===0) ctx.moveTo(x,y); else ctx.lineTo(x,y);
    }
    ctx.strokeStyle = 'rgba(96,165,250,0.95)';
    ctx.lineWidth = 2;
    ctx.stroke();

    ctx.lineTo(w-pad, h-pad);
    ctx.lineTo(pad, h-pad);
    ctx.closePath();
    ctx.fillStyle = 'rgba(96,165,250,0.08)';
    ctx.fill();

    const last = history[history.length-1];
    ctx.fillStyle = 'rgba(255,255,255,0.9)';
    ctx.font = '12px Arial';
    ctx.fillText(last.toString() + ' req/s', 8, 14);
  }

  async function fetchAndUpdate(){
    try {
      const r = await fetch(STATUS_URL, {cache:'no-store'});
      if (!r.ok) throw new Error('Status fetch failed: ' + r.status);
      const txt = await r.text();
      const parsed = parseStubStatus(txt);
      updateUI(parsed);
    } catch (err){
      console.error(err);
      lastUpdated.textContent = 'Ошибка: ' + err.message;
    }
  }

  function startPolling(){
    stopPolling();
    const iv = parseInt(pollSelect.value,10);
    if (iv > 0){
      pollTimer = setInterval(fetchAndUpdate, iv);
    }
  }

  function stopPolling(){
    if (pollTimer) { clearInterval(pollTimer); pollTimer = null; }
  }

  refreshBtn.addEventListener('click', fetchAndUpdate);
  pollSelect.addEventListener('change', startPolling);

  fetchAndUpdate();
  startPolling();

})();
