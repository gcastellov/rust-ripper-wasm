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

                let names = {
                    1: "Md5",
                    2: "Base64",
                    3: "Sha-256",
                    4: "Md4",
                    5: "Sha1",
                    6: "Ripemd-128",
                    7: "Ripemd-320",
                    8: "Whirlpool",
                    9: "Md2",
                    10: "Ripemd-160",
                    11: "Blake2b-512",
                    12: "Blake2s-256",
                    13: "Tiger",
                    14: "Shabal192",
                    15: "Shabal224",
                    16: "Shabal256",
                    17: "Shabal384",
                    18: "Shabal512",
                };

                return names[id];
            };

            const write_ciphers = (ciphers) => {
                divResultsContainer.innerHTML = "";
                if (ciphers.length > 0) {
                    divResults.classList.remove("hidden");
                    ciphers.forEach(item => {

                        let divRow = document.createElement("div");
                        divRow.className = "px-4 py-5 bg-white space-y-2 sm:p-2";

                        let divGrid = document.createElement("div");
                        divGrid.className = "grid grid-cols-5 gap-1";                        

                        let chunks = item.split("|");                        
                        let divAlgorithm = document.createElement("div");
                        divAlgorithm.className = "col-span-5 lg:col-span-1";

                        let algorithm = document.createElement("span");
                        algorithm.className = "text-sm font-medium text-black-700";
                        algorithm.innerText = getAlgorithm(chunks[0]);
                        divAlgorithm.appendChild(algorithm);

                        let divCipher = document.createElement("div");
                        divCipher.className = "break-words col-span-5 lg:col-span-4";

                        let content = document.createElement("span");
                        content.className = "text-sm font-medium text-gray-600";
                        content.innerHTML = chunks[1];

                        divCipher.appendChild(content);

                        divGrid.appendChild(divAlgorithm);
                        divGrid.appendChild(divCipher);
                        divRow.appendChild(divGrid);
                        divResultsContainer.appendChild(divRow);
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
            txtInput.addEventListener("keypress", (e) => {
                if (e.key == "Enter") {
                    run();
                }
            });
        });
    });
}