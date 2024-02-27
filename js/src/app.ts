import * as THREE from 'three';
import { GLTFLoader } from 'three/addons/loaders/GLTFLoader.js';
import { decode } from 'js-base64';

const loader = new GLTFLoader();

interface State {
    num: number,
    render: boolean,
    scene?: THREE.Scene,
}

let state: State = {
    num: 0,
    render: false,
    scene: undefined,
};

function call_ipc(data: any) {
    (window as any).ipc.postMessage(JSON.stringify(data));
}

function load_gltf(text: string): Promise<void> {
    return new Promise((resolve, reject) => {
        const gltf = JSON.parse(decode(text));
        loader.parse(
            gltf,
            '', 
            (gltf) => {
                if(state.scene) {
                    const mesh = gltf.scene;
                    mesh.traverse(child => {
                        if((child as any).isMesh) {
                            child.castShadow = true;
                            child.receiveShadow = true;
                        }
                    });

                    state.scene.clear();
                    add_lights();
                    state.scene.add(mesh);
                    resolve();
                }
            },
            (err) => reject(err)
        );
    });
}

function add_lights() {
    if(state.scene) {
        const fillLight1 = new THREE.HemisphereLight(0xffffff, 0x444444, 2.0);
        fillLight1.position.set(0, 300, 0);
        state.scene.add(fillLight1);

        const directionalLight = new THREE.DirectionalLight(0xffffff, 7.0);
        directionalLight.position.set(15, 30, 10);
        directionalLight.castShadow = true;
        directionalLight.shadow.camera.near = 0.01;
        directionalLight.shadow.camera.far = 500.0;
        directionalLight.shadow.camera.right = 60.0;
        directionalLight.shadow.camera.left = -60.0;
        directionalLight.shadow.camera.top = 60.0;
        directionalLight.shadow.camera.bottom = -60.0;
        directionalLight.shadow.mapSize.width = 2048;
        directionalLight.shadow.mapSize.height = 2048;
        directionalLight.shadow.radius = 4;
        directionalLight.shadow.bias = -0.00006;
        state.scene.add(directionalLight);
    }
}

(window as any).app = {
    init: () => {
        const canvas = document.getElementById('canvas')! as HTMLCanvasElement;

        const w = canvas.clientWidth;
        const h = canvas.clientHeight;
        
        state.scene = new THREE.Scene();

        const renderer = new THREE.WebGLRenderer({
            canvas,
            antialias: true,
        });
        renderer.shadowMap.enabled = true;
        renderer.shadowMap.type = THREE.VSMShadowMap;
        renderer.toneMapping = THREE.LinearToneMapping;    
        renderer.setPixelRatio(window.devicePixelRatio);
        renderer.setSize(w, h);

        add_lights();

        const geometry = new THREE.BoxGeometry( 3.0, 3.0, 3.0 );
        const material = new THREE.MeshBasicMaterial( { color: 0x00ff00 } );
        const cube = new THREE.Mesh( geometry, material );
        state.scene.add( cube );

        const camera = new THREE.PerspectiveCamera(75.0, w / h, 0.1, 1000);
        camera.position.set(0.0, 60.0, 60.0);

        function render() {
            cube.rotateX(0.01);
            cube.rotateY(0.01);
            renderer.render(state.scene!, camera);

            if(state.render) {
                state.render = false;

                const jpeg = canvas.toDataURL("image/jpeg", 0.9);

                call_ipc({
                    cmd: 'saveJpeg',
                    num: state.num,
                    image: jpeg,
                });
            }

            window.requestAnimationFrame(render);
        }

        window.requestAnimationFrame(render);
    },
    begin: (gltf: string) => {
        load_gltf(gltf).then(() => {
            state.num = 0;
            state.render = true;
        }).catch((err) => {
            console.error(err);
        });
    },
    next: (gltf: string) => {
        load_gltf(gltf).then(() => {
            state.num += 1;
            state.render = true;
        }).catch((err) => {
            console.error(err);
        });
    }
};
