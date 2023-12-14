import init, { run, initialize, reset, expand_cache } from "./pkg/graphing_calculator.js";
init().then(() => {
    //resize canvas if necessary
    const canvas = document.getElementById("canvas");
    let height = parseInt(getComputedStyle(canvas).height);
    if(height < canvas.height) {
        canvas.height = Math.min(height - 100, 700);
        canvas.width = Math.min(height - 100, 700);
    }

    //draw empty grid on canvas
    initialize([]);
    run(-5.0, 5.0, -5.0, 5.0);
    let currentView = [-5.0, 5.0, -5.0, 5.0];
    let dragging = false;
    let x = 0;
    let y = 0;
    let interval;
    let keepDrawing = false;

    //function to draw each function graph at each animation frame while needed (when dragging)
    let animate = () => {
        run(currentView[0], currentView[1], currentView[2], currentView[3]);
        if (keepDrawing) {
            requestAnimationFrame(() => {
                animate();
            });
        }
    }

    //event handlers for desktop
    canvas.addEventListener("mousedown", (e) => {
        //set drag start point
        x = e.offsetX / (canvas.width / (currentView[1] - currentView[0]));
        y = e.offsetY / (canvas.width / (currentView[3] - currentView[2]));
        dragging = true;
        //stop expanding cache
        clearInterval(interval);
        interval = null;
        keepDrawing = true;
        animate();
    })
    canvas.addEventListener("mousemove", (e) => {
        if (dragging) {
            //update the current visible graph viewport based on drag coordinates
            let scaledOffsetX = e.offsetX / (canvas.width / (currentView[1] - currentView[0]));
            let scaledOffsetY = e.offsetY / (canvas.height / (currentView[3] - currentView[2]));
            let dragX = x - scaledOffsetX;
            let dragY = y - scaledOffsetY;
            currentView[0] = currentView[0] + dragX;
            currentView[1] = currentView[1] + dragX;
            currentView[2] = currentView[2] - dragY;
            currentView[3] = currentView[3] - dragY;
            x = scaledOffsetX;
            y = scaledOffsetY;
        }
    })
    canvas.addEventListener("mouseup", () => {
        dragging = false;
        keepDrawing = false;
        if (!interval) {
            //once we stop dragging reset interval to expand our points cache
            interval = setInterval(() => { expand_cache(); }, 33);
        }
    })
    //same as mouseup
    canvas.addEventListener("mouseleave", () => {
        dragging = false;
        keepDrawing = false;
        if (!interval) {
            interval = setInterval(() => { expand_cache(); }, 33);
        }
    })
    //fixes bug caused when user attempts to drag and drop item onto drawing canvas
    canvas.addEventListener("dragover", () => {
        dragging = false;
        keepDrawing = false;
    })
    
    //touch events for mobile
    canvas.addEventListener("touchstart", (e) => {
        e.preventDefault();
        //only support one finger drag
        if(e.touches.length === 1) {
            //set drag start point
            let touch = e.touches[0];
            x = (touch.pageX - canvas.offsetLeft) / (canvas.width / (currentView[1] - currentView[0]));
            y = (touch.pageY - canvas.offsetTop) / (canvas.width / (currentView[3] - currentView[2]));
            dragging = true;
            //stop expanding cache
            clearInterval(interval);
            interval = null;
            keepDrawing = true;
            animate();
        }
    })
    canvas.addEventListener("touchmove", (e) => {
        e.preventDefault();
        if (dragging && e.touches.length === 1) {
            //update the current visible graph viewport based on drag coordinates
            let touch = e.touches[0];
            if (canvas !== document.elementFromPoint(touch.pageX, touch.pageY)) {
                dragging = false;
                keepDrawing = false;
                if (!interval) {
                    //once we stop dragging reset interval to expand our points cache
                    interval = setInterval(() => { expand_cache(); }, 33);
                }
            }
            let scaledOffsetX = (touch.pageX - canvas.offsetLeft) / (canvas.width / (currentView[1] - currentView[0]));
            let scaledOffsetY = (touch.pageY - canvas.offsetTop) / (canvas.height / (currentView[3] - currentView[2]));
            let dragX = x - scaledOffsetX;
            let dragY = y - scaledOffsetY;
            //multiply drag lengths by 1.25 for quicker dragging in mobile
            currentView[0] = currentView[0] + dragX;
            currentView[1] = currentView[1] + dragX;
            currentView[2] = currentView[2] - dragY;
            currentView[3] = currentView[3] - dragY;
            x = scaledOffsetX;
            y = scaledOffsetY;
        } 
    })
    canvas.addEventListener("touchend", (e) => {
        e.preventDefault();
        dragging = false;
        keepDrawing = false;
        if (!interval) {
            //once we stop dragging reset interval to expand our points cache
            interval = setInterval(() => { expand_cache(); }, 33);
        }
    })

    //zoom functionality
    let zoomIn = document.getElementById("zoom-in");
    let zoomOut = document.getElementById("zoom-out");
    let adjustButtonStyles = () => {
        let currentViewRange = currentView[1] - currentView[0];
        if (Math.ceil(currentViewRange) >= 40) {
            zoomOut.style.color = "gray";
        }
        else {
            zoomOut.style.color = "black";
        }
        if (Math.floor(currentViewRange) <= 10) {
            zoomIn.style.color = "gray";
        }
        else {
            zoomIn.style.color = "black";
        }
    }
    zoomOut.addEventListener("click", () => {
        let currentViewRange = currentView[1] - currentView[0];
        //only allow max graph viewport size of 40x40
        if (Math.ceil(currentViewRange) >= 40) {
            return;
        }
        //make sure our zooming is centered on our current view center
        let currentCenterX = (currentView[0] + currentView[1]) / 2;
        let currentCenterY = (currentView[2] + currentView[3]) / 2;
        currentView[0] = currentCenterX - currentViewRange;
        currentView[1] = currentCenterX + currentViewRange;
        currentView[2] = currentCenterY - currentViewRange;
        currentView[3] = currentCenterY + currentViewRange;
        adjustButtonStyles();
        keepDrawing = false;
        //redraw graphs for new viewport size
        run(currentView[0], currentView[1], currentView[2], currentView[3]);
        if (!interval) {
            //expand cache
            interval = setInterval(() => { expand_cache(); }, 33);
        }
    })
    zoomIn.addEventListener("click", () => {
        let currentViewRange = currentView[1] - currentView[0];
        //only allow min graph viewport size of 10x10
        if (Math.floor(currentViewRange) <= 10) {
            return;
        }
        let currentCenterX = (currentView[0] + currentView[1]) / 2;
        let currentCenterY = (currentView[2] + currentView[3]) / 2;
        currentView[0] = currentCenterX - currentViewRange / 4;
        currentView[1] = currentCenterX + currentViewRange / 4;
        currentView[2] = currentCenterY - currentViewRange / 4;
        currentView[3] = currentCenterY + currentViewRange / 4;
        adjustButtonStyles();
        keepDrawing = false;
        run(currentView[0], currentView[1], currentView[2], currentView[3]);
        if (!interval) {
            interval = setInterval(() => { expand_cache(); }, 33);
        }
    })

    //event handlers for function inputs
    let inputs = document.getElementsByClassName("function-input");
    for (let i = 0; i < inputs.length; ++i) {
        let el = inputs[i];
        el.addEventListener("keyup", () => {
            keepDrawing = false;
            let functions = [];
            for (let el of inputs) {
                //disregard empty inputs
                if (el.value !== "") {
                    functions.push(el.value);
                }
            }
            //completely reset all global app state, since we have new input, then reinitialize it based on new input
            reset();
            let result = initialize(functions);
            //draw new function graphs
            run(currentView[0], currentView[1], currentView[2], currentView[3]);
            if (!interval) {
                interval = setInterval(() => { expand_cache(); }, 33);
            }
            //indicate no error for this input
            el.className = "function-input";
            for (let r of result) {
                if (!r && el.value !== "") {
                    //indicate error for input if its expression is invalid
                    el.className = "function-input error";
                }
            }
        })
    }

    //resize canvas on window resize
    addEventListener("resize", () => {
        let height = parseInt(getComputedStyle(canvas).height);
        //only resize once difference hits 100px to avoid resizing too often
        if(Math.abs(height - canvas.height) > 100) {
            //set dimensions to 100 less than displayed dimensions, only allow canvas to go up to 700x700
            height = height - 100;
            canvas.height = Math.max(height, 700);
            canvas.width = Math.max(height, 700);
            run(currentView[0], currentView[1], currentView[2], currentView[3]);
        }
    })
});