import {Mutex} from "async-mutex";
const js = import("./node_modules/rust_ripper_wasm/rust_ripper_wasm.js");
const mem =  import("./node_modules/rust_ripper_wasm/rust_ripper_wasm_bg.wasm");

const txtWordProgress = document.getElementById("txtWordProgress");
const txtResult = document.getElementById("txtResult");
const lblVersion = document.getElementById("lblVersion");

lblVersion.innerHTML = APP_VERSION;

const debounce = (callback, delay) => {
    let timeout;
    return function() {
        clearTimeout(timeout);
        timeout = setTimeout(callback, delay);
    }
};

mem.then(m => {
    const memory = m.memory;

    js.then(async j => {

        const ckDictionaries = document.getElementsByName("ckDictionary");
        const rbAlgorithm = document.getElementsByName("rbAlgorithm");
        const txtWordListCount = document.getElementById("txtWordListCount");
        const txtPwdOutput = document.getElementById("txtPwdOutput");
        const txtElapsedTime = document.getElementById("txtElapsedTime");
        const txtLastWord = document.getElementById("txtLastWord");
        
        let mutex = new Mutex();
        let ripper = new j.HashRipper();

        const clean = () => {
            return new Promise((resolve, reject) => {
                txtResult.value = "";
                txtPwdOutput.value = "";
                txtWordProgress.value = "";
                txtElapsedTime.value = "";
                txtLastWord.value = "";
                resolve();
            });
        };

        const loop = () => {
            const found = ripper.check(500);
            const progress = ripper.get_progress();
            const lastWord =  ripper.get_last_word();
            txtElapsedTime.value = ripper.get_elapsed_seconds();
            txtLastWord.value = lastWord;
            txtWordProgress.value = progress;
            
            if (found) {
                txtPwdOutput.value = ripper.get_match();
                txtResult.value = "FOUND!!";
            } else if (progress < ripper.get_word_list_count()) {
                requestAnimationFrame(loop);
            } else {
                txtResult.value = "NOT FOUND!!";
            }
        };

        const execute = () => {
            clean()
                .then(run())
                .then(requestAnimationFrame(loop));
        };

        const run = () => {
            return new Promise((resolve, reject) => {
                const pwd = document.getElementById("txtPwdInput").value;
                const algorithm = getSelectedAlgorithm();
                ripper.set_algorithm(algorithm);
                ripper.set_input(pwd);
                ripper.start_matching();
                resolve();
            });
        };
       
        const getSelectedAlgorithm = () => {
            const algorithms = Array.from(rbAlgorithm);
            const selected =  algorithms.find(algorithm => algorithm.checked);
            return selected.value;
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
                .filter(dictionary => !ripper.has_dictionary(dictionary))
                .map(dictionary => {
                    return fetch(`./assets/${dictionary}`, headers)
                        .then(r => r.arrayBuffer())
                        .then(buffer => {
                            const encoding = getEncoding(dictionary);
                            const decoder = new TextDecoder(encoding);
                            return decoder.decode(buffer);
                        })
                        .then(text => ripper.add_dictionary(dictionary, text));
                });

            await Promise.all(promises);

            ripper.load_dictionaries(dictionaries);
            txtWordListCount.value = ripper.get_word_list_count();
            release();
        };

        await updateDictionarySelection();

        const button = document.getElementById("btnRun");
        button.addEventListener("click", execute);

        rbAlgorithm.forEach(element => element.addEventListener("change", clean));
        ckDictionaries.forEach(element => element.addEventListener("change", debounce(updateDictionarySelection, 2000)));
    });
});