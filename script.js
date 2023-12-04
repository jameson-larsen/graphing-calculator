import init, { run, initialize, reset, expand_cache } from "./pkg/graphing_calculator.js";
init().then(() => {
  initialize([]);
  run(-5.0, 5.0, -5.0, 5.0);
  let currentView = [-5.0, 5.0, -5.0, 5.0];
  let dragging = false;
  let x = 0;
  let y = 0;
  const canvas = document.getElementById("canvas");
  let interval;
  let interval2;
  canvas.addEventListener("mousedown", (e) => {
    x = e.offsetX / (canvas.width / (currentView[1] - currentView[0]));
    y = e.offsetY / (canvas.width / (currentView[3] - currentView[2]));
    dragging = true;
    clearInterval(interval2);
    interval2 = null;
    interval = setInterval(() => {
        run(currentView[0], currentView[1], currentView[2], currentView[3]);
    }, 33);
  })
  canvas.addEventListener("mousemove", (e) => {
    if(dragging) {
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
  canvas.addEventListener("mouseup", () =>  {
    dragging = false;
    clearInterval(interval);
    if(!interval2) {
        interval2 = setInterval(() => { expand_cache(); }, 50);
    }
  })
  canvas.addEventListener("mouseleave", () => {
    dragging = false;
    clearInterval(interval);
    if(!interval2) {
        interval2 = setInterval(() => { expand_cache(); }, 50);
    }
  })
  let zoomIn = document.getElementById("zoom-in");
  let zoomOut = document.getElementById("zoom-out");
  let adjustButtonStyles = () => {
    let currentViewRange = currentView[1] - currentView[0];
    if(Math.ceil(currentViewRange) >= 40) {
        zoomOut.style.color = "gray";
        zoomOut.style.cursor = "not-allowed";
    }
    else {
        zoomOut.style.color = "black";
        zoomOut.style.cursor = "pointer";
    }
    if(Math.floor(currentViewRange) <= 10) {
        zoomIn.style.color = "gray";
        zoomIn.style.cursor = "not-allowed";
    }
    else {
        zoomIn.style.color = "black";
        zoomIn.style.cursor = "pointer";
    }
  }
  zoomOut.addEventListener("click", () => {
    let currentViewRange = currentView[1] - currentView[0];
    if(Math.ceil(currentViewRange) >= 40) {
        return;
    }
    let currentCenterX = (currentView[0] + currentView[1]) / 2;
    let currentCenterY = (currentView[2] + currentView[3]) / 2;
    currentView[0] = currentCenterX - currentViewRange;
    currentView[1] = currentCenterX + currentViewRange;
    currentView[2] = currentCenterY - currentViewRange;
    currentView[3] = currentCenterY + currentViewRange;
    adjustButtonStyles();
    clearInterval(interval);
    run(currentView[0], currentView[1], currentView[2], currentView[3]);
    if(!interval2) {
        interval2 = setInterval(() => { expand_cache(); }, 50);
    }
  })
  zoomIn.addEventListener("click", () => {
    let currentViewRange = currentView[1] - currentView[0];
    if(Math.floor(currentViewRange) <= 10) {
        return;
    }
    let currentCenterX = (currentView[0] + currentView[1]) / 2;
    let currentCenterY = (currentView[2] + currentView[3]) / 2;
    currentView[0] = currentCenterX - currentViewRange / 4;
    currentView[1] = currentCenterX + currentViewRange / 4;
    currentView[2] = currentCenterY - currentViewRange / 4;
    currentView[3] = currentCenterY + currentViewRange / 4;
    adjustButtonStyles();
    clearInterval(interval);
    run(currentView[0], currentView[1], currentView[2], currentView[3]);
    if(!interval2) {
        interval2 = setInterval(() => { expand_cache(); }, 50);
    }
  })
  let inputs = document.getElementsByClassName("function-input");
  for(let i = 0; i < inputs.length; ++i) {
    let el = inputs[i];
    el.addEventListener("keyup", () => {
        clearInterval(interval);
        let functions = [];
        for(let el of inputs) {
            if(el.value !== "") {
                functions.push(el.value);
            }
        }
        reset();
        let result = initialize(functions);
        if(result.every(el => el)) {
            run(currentView[0], currentView[1], currentView[2], currentView[3]);
            if(!interval2) {
                interval2 = setInterval(() => { expand_cache(); }, 50);
            }
            el.className="function-input";
        }
        else {
            for(let r of result) {
                if(!r && el.value !== "") {
                    el.className = "function-input error";
                }
            }
        }
    })
  }
  canvas.addEventListener("dragover", () => {
    dragging = false;
    clearInterval(interval);
  })
});