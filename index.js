const rust =  import("./pkg/tomcraft");
const canvas = document.getElementById("drawTarget");
const gl = canvas.getContext("webgl", { antialias: true });

rust.then(m => {
    if (!gl) {
        alert("Failed to initializer WebGl");
        return;
    }

    const FPT_THROTTLE = 1000.0 / 30.0;
    const client = new m.GameClient();
    const initialTime = Date.now();
    var lastDrawTime = -1;

    function render() {
        window.requestAnimationFrame(render);
        const currTime = Date.now();

        if (currTime >= lastDrawTime + FPT_THROTTLE) {
            lastDrawTime = currTime;

            if(window.innerHeight != canvas.height || window.width != canvas.width) {
                canvas.height = window.innerHeight;
                canvas.clientHeight = window.innerHeight;
                canvas.style.height = window.innerHeight;

                canvas.width = window.innerWidth;
                canvas.clientWidth = window.innerWidth;
                canvas.style.width = window.innerWidth;

                gl.viewport(0, 0, window.innerWidth, window.innerHeight);
            }


            let elapsedTime = currTime - initialTime;

            client.update(elapsedTime, window.innerHeight, window.innerWidth);
            client.render();
        }

    }

    render();
}).catch(console.error);

   