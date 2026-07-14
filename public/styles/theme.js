/* Shared theme handling for standalone essay/paper pages.
   Reads the same localStorage key as the SPA so the choice
   follows the reader across the whole site. */
;(function () {
  var root = document.documentElement

  function current() {
    return root.classList.contains('light') ? 'light' : 'dark'
  }

  function apply(theme) {
    root.classList.toggle('light', theme === 'light')
    try {
      localStorage.setItem('theme', theme)
    } catch {
      /* persistence is best-effort */
    }
    render()
  }

  var wrap = document.createElement('div')
  wrap.className = 'theme-toggle'

  var buttons = ['dark', 'light'].map(function (theme) {
    var btn = document.createElement('button')
    btn.type = 'button'
    btn.dataset.theme = theme
    btn.setAttribute('aria-label', 'Switch to ' + theme + ' mode')
    btn.addEventListener('click', function () {
      apply(theme)
    })
    wrap.appendChild(btn)
    return btn
  })

  function render() {
    buttons.forEach(function (btn) {
      var active = btn.dataset.theme === current()
      btn.setAttribute('aria-pressed', active ? 'true' : 'false')
      btn.textContent = (active ? '■' : '□') + ' ' + btn.dataset.theme
    })
  }

  render()
  document.body.appendChild(wrap)
})()
