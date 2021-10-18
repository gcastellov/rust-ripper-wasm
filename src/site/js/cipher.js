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
                        return "Sha-256";
                    case "4":
                        return "Md4";
                    case "5":
                        return "Sha1";
                    case "6":
                        return "Ripemd-128";
                    case "7":
                        return "Ripemd-320";
                    case "8":
                        return "Whirlpool";
                    case "9":
                        return "Md2";
                    case "10":
                        return "Ripemd-160";
                    case "11":
                        return "Blake2b-512";
                    case "12":
                        return "Blake2s-256";
                    case "13":
                        return "Tiger";
                    default:
                        return id;
                }
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