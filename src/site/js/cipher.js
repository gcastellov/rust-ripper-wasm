const js = import("rust_ripper_wasm/rust_ripper_wasm.js");
const mem =  import("rust_ripper_wasm/rust_ripper_wasm_bg.wasm");

const nav = document.getElementById("nav-cipher");
if (nav !== null && nav !== "undefined") {
    mem.then(m => {
        js.then(async j => {
            const txtInput = document.getElementById("txtInput");
            const btnRun = document.getElementById("btnRun");
            const divResultsContainer = document.getElementById("results-container");
            const divResults = document.getElementById("results");

            const getAlgorithm = (id) => {
                switch(id) {
                    case "1":
                        return "Md5";
                    case "2":
                        return "Base64";
                    case "3":
                        return "Sha256";
                    case "4":
                        return "Md4";
                    case "5":
                        return "Sha1";
                    case "6":
                        return "Ripemd128";
                    case "7":
                        return "Ripemd320";
                    case "8":
                        return "Whirlpool";
                    case "9":
                        return "Md2";
                    case "10":
                        return "Ripemd160";
                }
            };

            const write_ciphers = (ciphers) => {
                divResultsContainer.innerHTML = "";
                if (ciphers.length > 0) {
                    divResults.classList.remove("hidden");
                    ciphers.forEach(item => {
                        let chunks = item.split("|");                        
                        let divAlgorithm = document.createElement("div");
                        divAlgorithm.className = "col-span-1 sm:col-span-1";
                        divAlgorithm.innerText = getAlgorithm(chunks[0]);

                        let divCipher = document.createElement("div");
                        divCipher.className = "col-span-1 sm:col-span-1";

                        let content = document.createElement("span");
                        content.innerHTML = chunks[1];

                        divCipher.appendChild(content);

                        divResultsContainer.appendChild(divAlgorithm);
                        divResultsContainer.appendChild(divCipher);
                    });
                } else if (!divResults.classList.contains("hidden")) {
                    divResults.classList.add("hidden");
                }
            };

            const run = () => {
                let cipher = new j.HashCipher();                
                cipher.set_word(txtInput.value);
                const result = cipher.get_ciphers();
                write_ciphers(result);
            };            

            btnRun.addEventListener("click", run);            
        });
    });
}