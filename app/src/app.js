const compiler = require('../../compiler/target/wasm32-unknown-unknown/release/compiler.js')

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
    const coordinates = {
        x: e.clientX - rect.left,
        y: e.clientY - rect.top,
        canvasWidth: rect.width,
        canvasHeight: rect.height
    }
    vm.postIOData('mouse', coordinates)
})
canvas.addEventListener('mousedown', e => {
    const rect = canvas.getBoundingClientRect()
    const data = {
        isDown: true,
        x: e.clientX - rect.left,
        y: e.clientY - rect.top,
        canvasWidth: rect.width,
        canvasHeight: rect.height
    }
    vm.postIOData('mouse', data)
    e.preventDefault()
})
canvas.addEventListener('mouseup', e => {
    const rect = canvas.getBoundingClientRect()
    const data = {
        isDown: false,
        x: e.clientX - rect.left,
        y: e.clientY - rect.top,
        canvasWidth: rect.width,
        canvasHeight: rect.height
    }
    vm.postIOData('mouse', data)
    e.preventDefault()
})

// Feed keyboard events as VM I/O events.
document.addEventListener('keydown', e => {
    // Don't capture keys intended for Blockly inputs.
    if (e.target !== document && e.target !== document.body) {
        return
    }
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

document.getElementById('green-flag').addEventListener('click', () => {
  vm.greenFlag()
})

document.getElementById('btn-load').addEventListener('click', () => {
  load()
})

const textarea = document.getElementById('source')
textarea.focus()

function load() {
  const source = textarea.value

  compiler.then(({ compile }) => {
    const project = compile(source)

    // Load the project in
    vm.fromJSON(project) /*{
      "targets": [
        {
          "id": "VW3I0)K5E]3[Q/VkKr=H ok",
          "name": "Stage",
          "isStage": true,
          "x": 0,
          "y": 0,
          "size": 100,
          "direction": 90,
          "draggable": false,
          "currentCostume": 0,
          "costume": {
            "name": "backdrop1",
            "bitmapResolution": 1,
            "rotationCenterX": 240,
            "rotationCenterY": 180,
            "skinId": 0,
            "dataFormat": "png",
            "assetId": "739b5e2a2435f6e1ec2993791b423146"
          },
          "costumeCount": 1,
          "visible": true,
          "rotationStyle": "all around",
          "blocks": {},
          "variables": {},
          "lists": {},
          "costumes": [
            {
              "name": "backdrop1",
              "bitmapResolution": 1,
              "rotationCenterX": 240,
              "rotationCenterY": 180,
              "skinId": 0,
              "dataFormat": "png",
              "assetId": "739b5e2a2435f6e1ec2993791b423146"
            }
          ],
          "sounds": [
            {
              "name": "pop",
              "format": "",
              "rate": 11025,
              "sampleCount": 258,
              "soundID": -1,
              "md5": "83a9787d4cb6f3b7632b4ddfebf74367.wav",
              "data": null,
              "dataFormat": "wav",
              "assetId": "83a9787d4cb6f3b7632b4ddfebf74367",
              "soundId": "2O)CpjMgYm%:R]M62r!z"
            }
          ]
        },
        {
          "id": "I1Zhgfstp;XZ~_g:a|qi",
          "name": "Sprite1",
          "isStage": false,
          "x": 0,
          "y": 0,
          "size": 100,
          "direction": 90,
          "draggable": false,
          "currentCostume": 0,
          "costume": {
            "name": "costume1",
            "bitmapResolution": 1,
            "rotationCenterX": 47,
            "rotationCenterY": 55,
            "skinId": 1,
            "dataFormat": "svg",
            "assetId": "09dc888b0b7df19f70d81588ae73420e"
          },
          "costumeCount": 2,
          "visible": true,
          "rotationStyle": "all around",
          "blocks": {
            "3!W8ZRuJs7iO!L1uWc4Y": {
              "id": "3!W8ZRuJs7iO!L1uWc4Y",
              "opcode": "event_whenflagclicked",
              "inputs": {},
              "fields": {},
              "next": "8[N{;dJbyxZ]JTlbYZ{1",
              "shadow": false,
              "x": 36,
              "y": 74.80000000000001,
              "topLevel": true,
              "parent": null
            },
            "8[N{;dJbyxZ]JTlbYZ{1": {
              "id": "8[N{;dJbyxZ]JTlbYZ{1",
              "opcode": "looks_say",
              "inputs": {
                "MESSAGE": {
                  "name": "MESSAGE",
                  "block": "r8Ps[?M_efbmnx|dV#]}",
                  "shadow": "r8Ps[?M_efbmnx|dV#]}"
                }
              },
              "fields": {},
              "next": null,
              "shadow": false,
              "parent": "3!W8ZRuJs7iO!L1uWc4Y"
            },
            "r8Ps[?M_efbmnx|dV#]}": {
              "id": "r8Ps[?M_efbmnx|dV#]}",
              "opcode": "text",
              "inputs": {},
              "fields": {
                "TEXT": {
                  "name": "TEXT",
                  "value": "Hello!"
                }
              },
              "next": null,
              "topLevel": false,
              "parent": "8[N{;dJbyxZ]JTlbYZ{1",
              "shadow": true
            },
            "B52dsbj/Gqp:1/jdlV*t": {
              "id": "B52dsbj/Gqp:1/jdlV*t",
              "opcode": "event_whenthisspriteclicked",
              "inputs": {},
              "fields": {},
              "next": "]Q15;dOR=CQR3sU=Lh}x",
              "shadow": false,
              "x": 40.5,
              "y": 336.6,
              "topLevel": true,
              "parent": null
            },
            "]Q15;dOR=CQR3sU=Lh}x": {
              "id": "]Q15;dOR=CQR3sU=Lh}x",
              "opcode": "looks_say",
              "inputs": {
                "MESSAGE": {
                  "name": "MESSAGE",
                  "block": "CszvP+X9]zU:W=PN^{e0",
                  "shadow": "CszvP+X9]zU:W=PN^{e0"
                }
              },
              "fields": {},
              "next": null,
              "shadow": false,
              "parent": "B52dsbj/Gqp:1/jdlV*t"
            },
            "CszvP+X9]zU:W=PN^{e0": {
              "id": "CszvP+X9]zU:W=PN^{e0",
              "opcode": "text",
              "inputs": {},
              "fields": {
                "TEXT": {
                  "name": "TEXT",
                  "value": "i mean me too thanks"
                }
              },
              "next": null,
              "topLevel": false,
              "parent": "]Q15;dOR=CQR3sU=Lh}x",
              "shadow": true
            }
          },
          "variables": {},
          "lists": {},
          "costumes": [
            {
              "name": "costume1",
              "bitmapResolution": 1,
              "rotationCenterX": 47,
              "rotationCenterY": 55,
              "skinId": 1,
              "dataFormat": "svg",
              "assetId": "09dc888b0b7df19f70d81588ae73420e"
            },
            {
              "name": "costume2",
              "bitmapResolution": 1,
              "rotationCenterX": 47,
              "rotationCenterY": 55,
              "skinId": 2,
              "dataFormat": "svg",
              "assetId": "3696356a03a8d938318876a593572843"
            }
          ],
          "sounds": [
            {
              "name": "meow",
              "format": "",
              "rate": 22050,
              "sampleCount": 18688,
              "soundID": -1,
              "md5": "83c36d806dc92327b9e7049a565c6bff.wav",
              "data": null,
              "dataFormat": "wav",
              "assetId": "83c36d806dc92327b9e7049a565c6bff",
              "soundId": "=]8[TR^jmtct2OF:Cw)U"
            }
          ]
        }
      ],
      "meta": {
        "semver": "3.0.0",
        "vm": "0.1.0",
        "agent": "scratch-wren"
      }
    })*/
  })
}

load()
