export default function Init () {
    return {
        onStart: () => {
            console.log("Loading...");
            console.time("trunk-initializer");
        },
        onProgress: ({current, total}) => {
            if (!total) {
                console.log("Loading...", current, "bytes");
                document.getElementById("load").innerHTML = `loading initiative...<br>downloading wasm binary ${current}/?`;
            } else {
                console.log("Loading...", Math.round((current/total) * 100), "%" );
                document.getElementById("load").innerHTML = `loading initiative...<br>downloading wasm binary ${current}/${total} (${Math.round((current/total) * 100)}%)`;
            }
        },
        onComplete: () => {
            console.log("Loading... done!");
            document.getElementById("load").innerHTML = "loading initiative...<br>done";
            console.timeEnd("trunk-initializer");
        },
        onSuccess: (wasm) => {
            console.log("Loading... successful!");
            document.getElementById("load").remove();
        },
        onFailure: (error) => {
            document.getElementById("load").innerHTML = `an error occurred!<br>${error}`;
            console.warn("Loading... failed!", error);
        }
    }
};