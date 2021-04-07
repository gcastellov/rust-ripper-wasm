import {Mutex} from "async-mutex";
const js = import("./node_modules/rust_ripper_wasm/rust_ripper_wasm.js");
const mem =  import("./node_modules/rust_ripper_wasm/rust_ripper_wasm_bg.wasm");

const txtWordProgress = document.getElementById("txtWordProgress");
const txtResult = document.getElementById("txtResult");
const lblVersion = document.getElementById("lblVersion");
const btnRun = document.getElementById("btnRun");
const btnCancel = document.getElementById("btnCancel");
const btnAll = document.getElementById("btnAll");

const DISABLED_ATTR = "disabled";

lblVersion.innerHTML = APP_VERSION;

const debounce = (callback, delay) => {
    let timeout;
    return function() {
        clearTimeout(timeout);
        timeout = setTimeout(callback, delay);
    }
};

let isCancelationRequested = false;

mem.then(m => {
    js.then(async j => {

        const ckDictionaries = document.getElementsByName("ckDictionary");
        const rbAlgorithm = document.getElementsByName("rbAlgorithm");
        const txtWordListCount = document.getElementById("txtWordListCount");
        const txtPwdOutput = document.getElementById("txtPwdOutput");
        const txtElapsedTime = document.getElementById("txtElapsedTime");
        const txtLastWord = document.getElementById("txtLastWord");
        
        let mutex = new Mutex();
        let dictionaryManager = new j.DictionaryManager();
        let ripper = null;

        const getSelectedAlgorithm = () => {
            const algorithms = Array.from(rbAlgorithm);
            const selected =  algorithms.find(algorithm => algorithm.checked);
            return selected.value;
        };
        
        const clean = (from) => {
            return new Promise((resolve, reject) => {
                txtResult.value = "";
                txtPwdOutput.value = "";
                txtWordProgress.value = "";
                txtElapsedTime.value = "";
                txtLastWord.value = "";
                resolve();
            });
        };

        const loop = async () => {
            const found = ripper.check(500);
            const progress = ripper.get_progress();
            const lastWord =  ripper.get_last_word();
            txtElapsedTime.value = ripper.get_elapsed_seconds();
            txtLastWord.value = lastWord;
            txtWordProgress.value = progress;

            if (!found && isCancelationRequested === false && ripper.is_checking()) {
                return requestAnimationFrame(loop);
            }
            
            if (found) {
                txtPwdOutput.value = ripper.get_match();
                txtResult.value = "FOUND!!";
            } else if (isCancelationRequested) {
                txtResult.value = "CANCELLED";
            } else {
                txtResult.value = "NOT FOUND!!";
            }

            await unblock();
        };

        const selectAll = async () => {
            const ckDictionaries = document.getElementsByName("ckDictionary");
            ckDictionaries.forEach(dictionary => {
                dictionary.checked = true;
            });

            await updateDictionarySelection();
        };

        const cancel = async () => {
            isCancelationRequested = true;
            await unblock();
        };

        const execute = () => {
            clean()
                .then(block())
                .then(run())
                .then(requestAnimationFrame(loop));
        };

        const unblock = () => {
            return new Promise((resolve, reject) => { 
                document.getElementsByName("rbAlgorithm").forEach(radio => {
                    radio.removeAttribute(DISABLED_ATTR);
                });
                document.getElementsByName("ckDictionary").forEach(check => {
                    check.removeAttribute(DISABLED_ATTR);
                });
                btnRun.removeAttribute(DISABLED_ATTR);
                btnAll.removeAttribute(DISABLED_ATTR);
                resolve();
            });
        };

        const block = () => {
            return new Promise((resolve, reject) => { 
                document.getElementsByName("rbAlgorithm").forEach(radio => {
                    radio.setAttribute(DISABLED_ATTR, "true");
                });
                document.getElementsByName("ckDictionary").forEach(check => {
                    check.setAttribute(DISABLED_ATTR, "true");
                });
                btnRun.setAttribute(DISABLED_ATTR, "true");
                btnAll.setAttribute(DISABLED_ATTR, "true");
                resolve();
            });
        }

        const run = () => {
            return new Promise((resolve, reject) => {
                isCancelationRequested = false;
                const pwd = document.getElementById("txtPwdInput").value;
                const algorithm = getSelectedAlgorithm();

                if (algorithm == "100") {
                    let lucky = new j.LuckyRipper(dictionaryManager);
                    ripper = lucky;                    
                } else {
                    let hasher = new j.HashRipper(dictionaryManager);
                    hasher.set_algorithm(algorithm);
                    ripper = hasher;
                }

                ripper.set_input(pwd);
                ripper.start_matching();
                resolve();
            });
        };
       
        const getSelectedDictionaries = () => {
            const dictionaries = Array.from(ckDictionaries);
            return dictionaries
                .filter(dictionary => {
                    return dictionary.checked;
                })
                .map(dictionary => dictionary.value);
        };

        const getEncoding = (dictionary) => {
            switch (dictionary) {
                case "russian.txt":
                    return "Windows-1251";
                case "spanish.txt":
                case "french.txt":
                case "czech.txt":
                case "finnish.txt":
                case "swedish.txt":
                case "german.txt":
                    return "Windows-1252";
                default:
                    return "utf-8";
            }
        };

        const updateDictionarySelection = async () => {
            const release = await mutex.acquire();
            const dictionaries = getSelectedDictionaries();
            var headers = new Headers();
            headers.append('Content-Type','text/plain; charset=UTF-16');
            const promises = dictionaries
                .filter(dictionary => !dictionaryManager.has_dictionary(dictionary))
                .map(dictionary => {
                    return fetch(`./assets/${dictionary}`, headers)
                        .then(r => r.arrayBuffer())
                        .then(buffer => {
                            const encoding = getEncoding(dictionary);
                            const decoder = new TextDecoder(encoding);
                            return decoder.decode(buffer);
                        })
                        .then(text => dictionaryManager.add_dictionary(dictionary, text));
                });

            await Promise.all(promises);

            dictionaryManager.load_dictionaries(dictionaries);
            txtWordListCount.value = dictionaryManager.get_word_list_count();
            release();
        };

        await updateDictionarySelection();

        btnRun.addEventListener("click", execute);
        btnAll.addEventListener("click", selectAll);
        btnCancel.addEventListener("click", cancel);
        rbAlgorithm.forEach(element => element.addEventListener("change", clean));
        ckDictionaries.forEach(element => element.addEventListener("change", debounce(updateDictionarySelection, 2000)));
    });
});