window.addEventListener('DOMContentLoaded', init);

function init() {
    const canvasElement = document.querySelector('#threeJsViewer');
    const renderer = new THREE.WebGLRenderer({
        canvas: canvasElement
    });

    const scene = new THREE.Scene();

    const camera = new THREE.PerspectiveCamera(45, 1.0, 1, 1000000);
    camera.position.set(0, 0, +1000);

    const controls = new THREE.OrbitControls(camera, canvasElement);

    const light = new THREE.DirectionalLight(0xFFFFFF, 1);
    light.position.set(0, 0, 1000000);
    scene.add(light);
    const ambientLight = new THREE.AmbientLight(0xFFFFFF, 0.3);
    scene.add(ambientLight);

    const invoke = window.__TAURI__.invoke;

    let st_bridge;
    invoke('read_st_bridge', {fileName: '/Users/Kota/rust/stb-viewer/steel_standard_model.stb'}).then((stb) => st_bridge = stb);
    invoke('members', {stBridge: st_bridge}).then((v) => {console.log(v)});

    const points = [];

    const geometry = THREE.BufferGeometry().setFromPoints(points);

    const material = new THREE.LineBasicMaterial({
        color: 0xFF0000
    });

    const line = new THREE.Line(geometry, material);
    scene.add(line);

    const meshFloor = new THREE.Mesh(
        new THREE.BoxGeometry(10000, 10000, 0.1),
        new THREE.MeshStandardMaterial({color: 0x808080, roughness: 0.0})
    );
    scene.add(meshFloor);

    const lightHelper = new THREE.DirectionalLightHelper(light);
    scene.add(lightHelper);
    const helper = new THREE.AxesHelper(1000);
    scene.add(helper);

    tick();

    function tick() {
        requestAnimationFrame(tick);
        renderer.render(scene, camera);
    }

    onResize();

    window.addEventListener('resize', onResize);

    function onResize() {
        const width = window.innerWidth;
        const height = window.innerHeight;

        renderer.setPixelRatio(window.devicePixelRatio);
        renderer.setSize(width, height);

        camera.aspect = width / height;
        camera.updateProjectionMatrix();
    }
}
