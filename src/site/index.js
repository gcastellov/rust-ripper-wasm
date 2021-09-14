import {Mutex} from "async-mutex";
const js = import("./node_modules/rust_ripper_wasm/rust_ripper_wasm.js");
const mem =  import("./node_modules/rust_ripper_wasm/rust_ripper_wasm_bg.wasm");

const DISABLED_ATTR = "disabled";
const elements = {
    TXT_PROGRESS: "txtWordProgress",
    TXT_RESULT: "txtResult",
    LBL_VERSION: "lblVersion",
    BTN_RUN: "btnRun",
    BTN_CANCEL: "btnCancel",
    BTN_ALL: "btnAll",
    CK_DICTIONARY: "ckDictionary",
    RB_ALGORITHM: "rbAlgorithm",
    TXT_WORD_COUNT: "txtWordListCount",
    TXT_OUTPUT: "txtPwdOutput",
    TXT_INPUT: "txtPwdInput",
    TXT_ELAPSED_TIME: "txtElapsedTime",
    TXT_LAST_WORD: "txtLastWord",
};

const events = {
    CLICK: "click",
    CHANGE: "change",
    KEYUP: "keyup",
};

const dictionaries = {
    RUSSIAN: "russian.txt",
    SPANISH: "spanish.txt",
    FRENCH: "french.txt",
    CZECH: "czech.txt",
    FINNISH: "finnish.txt",
    SWEDISH: "swedish.txt",
    GERMAN: "german.txt",
};

const encodings = {
    WIN_1251: "Windows-1251",
    WIN_1252: "Windows-1252",
    UTF_8: "utf-8",
};

const txtWordProgress = document.getElementById(elements.TXT_PROGRESS);
const txtResult = document.getElementById(elements.TXT_RESULT);
const lblVersion = document.getElementById(elements.LBL_VERSION);
const btnRun = document.getElementById(elements.BTN_RUN);
const btnCancel = document.getElementById(elements.BTN_CANCEL);
const btnAll = document.getElementById(elements.BTN_ALL);
let isCancelationRequested = false;
lblVersion.innerHTML = APP_VERSION;

const debounce = (callback, delay) => {
    let timeout;
    return function() {
        clearTimeout(timeout);
        timeout = setTimeout(callback, delay);
    }
};

mem.then(m => {
    js.then(async j => {

        let ripperCache = {
            hash: null,
            lucky: null,
            current: null,
            getInstance(algorithm) {
                let self = this;
                if (algorithm === "100" && (self.current == null || j.HashRipper.prototype.isPrototypeOf(self.current))) {                    
                    if (self.lucky == null) {
                        self.lucky = new j.LuckyRipper(dictionaryManager);
                    }
                    self.current = self.lucky;
                } else if (algorithm !== "100" && (self.current == null || j.LuckyRipper.prototype.isPrototypeOf(self.current))) {
                    if (self.hash == null) {
                        self.hash = new j.HashRipper(dictionaryManager);
                    }
                    self.current = self.hash;
                }
                
                if (j.HashRipper.prototype.isPrototypeOf(self.current)) {
                    self.hash.set_algorithm(algorithm);                    
                }

                return self.current;
            },
        };

        const ckDictionaries = document.getElementsByName(elements.CK_DICTIONARY);
        const rbAlgorithms = document.getElementsByName(elements.RB_ALGORITHM);
        const txtWordListCount = document.getElementById(elements.TXT_WORD_COUNT);
        const txtPwdOutput = document.getElementById(elements.TXT_OUTPUT);
        const txtElapsedTime = document.getElementById(elements.TXT_ELAPSED_TIME);
        const txtLastWord = document.getElementById(elements.TXT_LAST_WORD);
        const txtPassword = document.getElementById(elements.TXT_INPUT);
        
        let mutex = new Mutex();
        let dictionaryManager = new j.DictionaryManager();
        let ripper = null;
        let manualDictionarySelection = [];

        const getSelectedAlgorithm = () => {
            const algorithms = Array.from(rbAlgorithms);
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
            ckDictionaries.forEach(dictionary => {
                dictionary.checked = true;
            });

            await updateDictionarySelection();
        };

        const enableCancel = () => {
            return new Promise((resolve, reject) => { 
                btnCancel.removeAttribute(DISABLED_ATTR);
                btnCancel.classList.replace("cursor-not-allowed", "hover:bg-gray-500");
                resolve();
            });
        };

        const disableCancel = () => {
            return new Promise((resolve, reject) => { 
                btnCancel.setAttribute(DISABLED_ATTR, "true");
                btnCancel.classList.replace("hover:bg-gray-500", "cursor-not-allowed");
                resolve();
            });
        };

        const cancel = () => {
            isCancelationRequested = true;
            unblock().then(disableCancel());
        };

        const execute = () => {
            clean()
                .then(block())
                .then(enableCancel())
                .then(run())
                .then(requestAnimationFrame(loop));
        };

        const enableDisableExecution = async (e) => {
            const pwd = txtPassword.value;
            if (pwd) {
                btnRun.removeAttribute(DISABLED_ATTR);
                btnRun.classList.replace("cursor-not-allowed", "hover:bg-indigo-700");
                btnRun.classList.replace("bg-indigo-400", "bg-indigo-600");
            } else {
                btnRun.setAttribute(DISABLED_ATTR, "true");
                btnRun.classList.replace("hover:bg-indigo-700", "cursor-not-allowed");
                btnRun.classList.replace("bg-indigo-600", "bg-indigo-400");

                await disableCancel();
            }
        };

        const unblock = () => {
            return new Promise((resolve, reject) => { 
                rbAlgorithms.forEach(radio => radio.removeAttribute(DISABLED_ATTR));
                ckDictionaries.forEach(check => check.removeAttribute(DISABLED_ATTR));
                btnRun.removeAttribute(DISABLED_ATTR);
                btnAll.removeAttribute(DISABLED_ATTR);                
                btnRun.classList.replace("cursor-not-allowed", "hover:bg-indigo-700");
                btnAll.classList.replace("cursor-not-allowed", "hover:bg-gray-500");
                resolve();
            });
        };

        const block = () => {
            return new Promise((resolve, reject) => { 
                rbAlgorithms.forEach(radio => radio.setAttribute(DISABLED_ATTR, "true"));
                ckDictionaries.forEach(check => check.setAttribute(DISABLED_ATTR, "true"));
                btnRun.setAttribute(DISABLED_ATTR, "true");
                btnAll.setAttribute(DISABLED_ATTR, "true");
                btnRun.classList.replace("hover:bg-indigo-700", "cursor-not-allowed");
                btnAll.classList.replace("hover:bg-gray-500", "cursor-not-allowed");
                resolve();
            });
        };

        const run = () => {
            return new Promise((resolve, reject) => {
                isCancelationRequested = false;
                const pwd = txtPassword.value;
                const algorithm = getSelectedAlgorithm();
                ripper = ripperCache.getInstance(algorithm);
                ripper.set_input(pwd);
                ripper.start_matching();
                resolve();
            });
        };
       
        const getSelectedDictionaries = () => {
            const selDictionaries = Array.from(ckDictionaries);
            return selDictionaries
                .filter(dictionary => {
                    return dictionary.checked;
                })
                .map(dictionary => dictionary.value);
        };

        const getEncoding = (dictionary) => {
            switch (dictionary) {
                case dictionaries.RUSSIAN:
                    return encodings.WIN_1251;
                case dictionaries.SPANISH:
                case dictionaries.FRENCH:
                case dictionaries.CZECH:
                case dictionaries.FINNISH:
                case dictionaries.SWEDISH:
                case dictionaries.GERMAN:
                    return encodings.WIN_1252;
                default:
                    return encodings.UTF_8;
            }
        };

        const updateDictionarySelection = async () => {
            const release = await mutex.acquire();
            let selDictionaries = getSelectedDictionaries();
            var headers = new Headers();
            const promises = selDictionaries
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

            if (manualDictionarySelection) {
                selDictionaries = selDictionaries.concat(manualDictionarySelection);
            }

            dictionaryManager.load_dictionaries(selDictionaries);
            txtWordListCount.value = dictionaryManager.get_word_list_count();
            release();
        };

        const createManualDictionaryEntry = (file) => {
            const dictionaryList = document.getElementById('dictionary-list');
            const dictionaryEntry = document.createElement('li');
            dictionaryEntry.className = 'dictionary-entry'
            const dictionaryName = document.createElement('span');
            dictionaryName.innerHTML = file.name;
            const btnRemove = document.createElement('a');
            btnRemove.setAttribute('href', '#');            
            btnRemove.innerHTML = 'x';            
            dictionaryEntry.appendChild(dictionaryName);
            dictionaryEntry.appendChild(btnRemove);
            dictionaryList.appendChild(dictionaryEntry);
            btnRemove.addEventListener('click', async (e) => {
                const index = manualDictionarySelection.indexOf(dictionaryEntry.innerHTML);
                manualDictionarySelection.splice(index, 1);
                dictionaryList.removeChild(dictionaryEntry);
                if (manualDictionarySelection.length === 0) {
                    const separator = document.getElementsByClassName('separator')[0];
                    dictionaryList.removeChild(separator);
                }

                await updateDictionarySelection();
            });
        };

        const createDictionaryEntrySeparator = () => {
            const dictionaryEntry = document.createElement('li');
            dictionaryEntry.className = 'separator';
            const separator = document.createElement('hr');
            dictionaryEntry.appendChild(separator);
            document.getElementById('dictionary-list').appendChild(dictionaryEntry);
        };

        document.querySelector("#file-input").addEventListener('change', (e) => {
            let all_files = e.target.files;
            if(all_files.length == 0) {
                alert('Error : No file selected');
                return;
            }
        
            let file = all_files[0];
            let allowed_types = [ 'text/plain' ];
            if(allowed_types.indexOf(file.type) == -1) {
                alert('Error : Incorrect file type');
                return;
            }

            let max_size_allowed = 100*1024*1024
            if(file.size > max_size_allowed) {
                alert('Error : Exceeded size 100MB');
                return;
            }
        
            let reader = new FileReader();
        
            reader.addEventListener('load', async (e) => {
                if (manualDictionarySelection.indexOf(file.name) == -1) {
                    if (manualDictionarySelection.length == 0) {
                        createDictionaryEntrySeparator();
                    }
                    createManualDictionaryEntry(file);
                    manualDictionarySelection.push(file.name);
                }
                if (!dictionaryManager.has_dictionary(file.name)) {
                    let text = e.target.result;
                    dictionaryManager.add_dictionary(file.name, text);
                }
    
                await updateDictionarySelection();
            });
        
            reader.addEventListener('error', () => {
                alert('Error : Failed to read file');
            });
        
            reader.readAsText(file);
        });

        await updateDictionarySelection();
        await enableDisableExecution();

        btnRun.addEventListener(events.CLICK, execute);
        btnAll.addEventListener(events.CLICK, selectAll);
        btnCancel.addEventListener(events.CLICK, cancel);
        txtPassword.addEventListener(events.CHANGE, enableDisableExecution);
        txtPassword.addEventListener(events.KEYUP, enableDisableExecution);
        rbAlgorithms.forEach(element => element.addEventListener(events.CHANGE, clean));
        ckDictionaries.forEach(element => element.addEventListener(events.CHANGE, debounce(updateDictionarySelection, 2000)));
    });
});