const js = import("./node_modules/rust_ripper_wasm/rust_ripper_wasm.js");
const mem =  import("./node_modules/rust_ripper_wasm/rust_ripper_wasm_bg.wasm");

const txtWordProgress = document.getElementById("txtWordProgress");
const txtResult = document.getElementById("txtResult");

mem.then(m => {
    const memory = m.memory;

    js.then(async j => {

        const ckDictionaries = document.getElementsByName("ckDictionary");
        const rbAlgorithm = document.getElementsByName("rbAlgorithm");
        const txtWordListCount = document.getElementById("txtWordListCount");
        const txtPwdOutput = document.getElementById("txtPwdOutput");
        const txtElapsedTime = document.getElementById("txtElapsedTime");
        var ripper = new j.Ripper();

        const clean = () => {
            return new Promise((resolve, reject) => {
                txtResult.value = "";
                txtPwdOutput.value = "";
                txtWordProgress.value = "";
                txtElapsedTime.value = "";
                resolve();
            });
        };

        const execute = () => {
            clean()
                .then(run())
                .then(() => {
                    txtWordProgress.value = ripper.get_progress();
                    txtElapsedTime.value = ripper.get_elapsed_seconds();
                    const match = ripper.get_match();
                    if (match === "") {
                        txtResult.value = "NOT FOUND!!";
                    }
                    else {
                        txtPwdOutput.value = match;
                        txtResult.value = "FOUND!!";
                    }
                });
        };

        const run = () => {
            return new Promise((resolve, reject) => {
                const pwd = document.getElementById("txtPwdInput").value;
                const algorithm = getSelectedAlgorithm();
                ripper.set_algorithm(algorithm);
                ripper.set_input(pwd);
                ripper.try_match();
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

        const updateDictionarySelection = async () => {
   
            const dictionaries = getSelectedDictionaries();
            const promises = dictionaries
                .filter(dictionary => !ripper.has_dictionary(dictionary))
                .map(dictionary => {
                    return fetch(`./assets/${dictionary}`)
                        .then(r => r.text())
                        .then(text => ripper.add_dictionary(dictionary, text));
                });

            await Promise.all(promises);

            ripper.load_dictionaries(dictionaries);
            txtWordListCount.value = ripper.get_word_list_count();
        };

        await updateDictionarySelection();

        const button = document.getElementById("btnRun");
        button.addEventListener("click", execute);

        rbAlgorithm.forEach(element => element.addEventListener("change", clean));
        ckDictionaries.forEach(element => element.addEventListener("change", updateDictionarySelection));
    });
});