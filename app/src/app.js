const compiler = require('../../compiler/target/wasm32-unknown-unknown/release/bosh_compiler.js')

const VirtualMachine = require('scratch-vm')
const Renderer = require('scratch-render')
const AudioEngine = require('scratch-audio')
const ScratchStorage = require('scratch-storage')
const { AssetType } = ScratchStorage

const canvas = document.getElementById('scratch-stage')
const vm = new VirtualMachine()
const renderer = new Renderer(canvas)
const audioEngine = new AudioEngine()
const storage = new ScratchStorage()

storage.addWebSource([AssetType.ImageVector, AssetType.ImageBitmap, AssetType.Sound], asset =>
  'https://cdn.assets.scratch.mit.edu/' +
    'internalapi/asset/' +
    asset.assetId +
    '.' +
    asset.dataFormat +
    '/get/'
)

vm.attachStorage(storage)
vm.attachRenderer(renderer)
vm.attachAudioEngine(audioEngine)

// Feed mouse events as VM I/O events.
document.addEventListener('mousemove', e => {
  const rect = canvas.getBoundingClientRect();
  vm.postIOData('mouse', {
    x: e.clientX - rect.left,
    y: e.clientY - rect.top,
    canvasWidth: rect.width,
    canvasHeight: rect.height
  })
})
canvas.addEventListener('mousedown', e => {
  const rect = canvas.getBoundingClientRect()
  vm.postIOData('mouse', {
    isDown: true,
    x: e.clientX - rect.left,
    y: e.clientY - rect.top,
    canvasWidth: rect.width,
    canvasHeight: rect.height
  })

  e.preventDefault()
})
canvas.addEventListener('mouseup', e => {
  const rect = canvas.getBoundingClientRect()
  vm.postIOData('mouse', {
    isDown: false,
    x: e.clientX - rect.left,
    y: e.clientY - rect.top,
    canvasWidth: rect.width,
    canvasHeight: rect.height
  })

  e.preventDefault()
})

// Feed keyboard events as VM I/O events.
document.addEventListener('keydown', e => {
  vm.postIOData('keyboard', {
    keyCode: e.keyCode,
    isDown: true
  })

  e.preventDefault()
})
document.addEventListener('keyup', e => {
  // Always capture up events,
  // even those that have switched to other targets.
  vm.postIOData('keyboard', {
    keyCode: e.keyCode,
    isDown: false
  })
  // E.g., prevent scroll.
  if (e.target !== document && e.target !== document.body) {
    e.preventDefault()
  }
})

// Go!!
vm.start()

const textarea = document.getElementById('source')
const cm = CodeMirror.fromTextArea(textarea, {
  lineNumbers: true,
  lineWrapping: true,
  autofocus: true,
  mode: 'scheme',

  indentWithTabs: true, // sorry not sorry
  tabSize: 2,
})

cm.on('change', () => load())

function load() {
  const source = cm.getValue()

  compiler.then(({ compile }) => {
    const project = compile(source)

    vm.clear()

    if (project.Fail) {
      const error = project.Fail

      document.getElementById('parse-error').style.display = 'block'
      document.getElementById('parse-error').innerText = error

      return
    } else {
      document.getElementById('parse-error').style.display = 'none'
    }

    // Load the project in
    vm.fromJSON(project.Tree)

    // XXX: doesn't seem to be any better way to detect onload yet...
    setTimeout(() => {
      vm.greenFlag()
    }, 100)
  })
}

load()
