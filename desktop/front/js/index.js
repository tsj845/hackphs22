const cp = require("child_process");

cp.execSync("cargo build --release");

/**@type {HTMLDivElement} */
const equationdiv = document.getElementById("equation").children[0];
/**@type {HTMLSpanElement} */
const cleftText = equationdiv.children[0];
/**@type {HTMLSpanElement} */
const cmidText = equationdiv.children[1];
/**@type {HTMLSpanElement} */
const crightText = equationdiv.children[2];

/**@type {HTMLSpanElement} */
const expression = document.getElementById("latex-expression");
/**@type {HTMLSpanElement}*/
const code = document.getElementById("latex-code");

let curpos = 0;
let clength = 1;

/**
 * inserts the given char into the equation at the given pos
 * @param {String} char
 * @param {Number} pos
 */
function insertCharacter(char, pos) {
    if (pos <= curpos) {
        cleftText.textContent = cleftText.textContent.substring(0, pos) + char + cleftText.textContent.substring(pos+1);
    } else {
        crightText.textContent = crightText.textContent.substring(0, pos-(clength-crightText.textContent.length)) + char + crightText.textContent.substring(pos-(clength-crightText.textContent.length)+1);
    }
    clength += char.length;
    if (pos <= curpos) {
        curpos += char.length;
    }
}

/**
 * deletes the char at the specified pos from the equation
 * @param {Number} pos
 */
function deleteChar(pos) {
    if (pos < curpos) {
        cleftText.textContent = cleftText.textContent.substring(0, pos) + cleftText.textContent.substring(pos+1);
    } else if (pos === curpos) {
        cmidText.textContent = crightText.textContent[0];
        crightText.textContent = crightText.textContent.substring(1);
    } else {
        crightText.textContent = crightText.textContent.substring(0, pos-(clength-crightText.textContent.length)) + crightText.textContent.substring(pos-(clength-crightText.textContent.length)+1);
    }
    clength -= 1;
    if (pos <= curpos) {
        curpos -= 1;
    }
}

/**
 * moves the cursor by the specified delta
 * @param {Number} delta
 */
function mvCur(delta) {
    if (curpos + delta >= clength || curpos + delta < 0 || delta === 0) {
        return;
    }
    curpos += delta;
    if (delta > 0) {
        cleftText.textContent = cleftText.textContent + cmidText.textContent;
        cmidText.textContent = crightText.textContent[0];
        crightText.textContent = crightText.textContent.substring(1);
    } else {
        crightText.textContent = cmidText.textContent + crightText.textContent;
        cmidText.textContent = cleftText.textContent[cleftText.textContent.length-1];
        cleftText.textContent = cleftText.textContent.substring(0, cleftText.textContent.length-1);
    }
}

function regenMath() {
    cp.exec(`./target/release/hackphs22 "${cleftText.textContent+cmidText.textContent+crightText.textContent}"`, (_, out, err) => {
        expression.textContent = `$$${out.split("\n")[1]}$$`;
        code.textContent = `${out.split("\n")[1]}`;
        if (out.split("\n")[1].length) {
            code.hidden = false;
            expression.hidden = false;	    
        }
        MathJax.typesetPromise();
    });
}

document.addEventListener("keyup", (e) => {
    let key = e.key;
    if (key.length > 1) {
        if (key === "Backspace") {
            deleteChar(curpos-1);
        } else if (key === "ArrowLeft") {
            mvCur(-1);
        } else if (key === "ArrowRight") {
            mvCur(1);
        } else if (key === "Enter") {
            regenMath();
        }
        return;
    }
    if (e.shiftKey) {
        key = key.toUpperCase();
    }
    insertCharacter(key, curpos);
});

// document.addEventListener("keyup", (e) => {
//     console.log(e.code.toString());
// });
