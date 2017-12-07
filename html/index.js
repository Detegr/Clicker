function imports() {
	var ctx = canvas.getContext("2d");
	function clear() {
		ctx.fillStyle = "black";
		ctx.fillRect(0, 0, canvas.width, canvas.height);
	}
	function draw_clicks(clicks) {
		ctx.fillStyle = "yellow";
		ctx.font = "18pt Arial";
		ctx.fillText("Points: " + clicks, 0, 30);
	}
	function draw_plus_one(x, y, opacity) {
		ctx.fillStyle = "rgba(0, 255, 0, " + opacity + ")";
		ctx.font = "32pt Arial";
		ctx.fillText("+1", x, y);
	}
	return {
		clear,
		draw_clicks,
		draw_plus_one,
	};
}

fetch("clicker.wasm")
	.then(response => response.arrayBuffer())
	.then(bytes => WebAssembly.instantiate(bytes, { env: imports() }))
	.then(results =>{
		let module = results.instance.exports;

		function resize() {
			canvas.width = window.innerWidth;
			canvas.height = window.innerHeight;
			module.resize(canvas.width, canvas.height);
		}

		if (/Android|webOS|iPhone|iPad|iPod|BlackBerry|BB|PlayBook|IEMobile|Windows Phone|Kindle|Silk|Opera Mini/i.test(navigator.userAgent)) {
		} else {
			window.addEventListener('resize', () => {
				resize();
			});
		}

		window.addEventListener('click', (e) => {
			module.click(e.screenX, e.screenY);
		});

		window.addEventListener('touchstart', (e) => {
			module.click(e.pageX, e.pageY);
		});

		let start = null;
		let prevTimestamp = null;
		let drawAndUpdate = (timestamp) => {
			if (!prevTimestamp) {
				start = timestamp;
				prevTimestamp = timestamp;
				requestAnimationFrame(drawAndUpdate);
				return;
			}

			let progress = (timestamp - prevTimestamp);
			module.update(progress);
			module.draw();

			prevTimestamp = timestamp;
			requestAnimationFrame(drawAndUpdate);
		};

		module.init();
		resize();
		drawAndUpdate();
});
