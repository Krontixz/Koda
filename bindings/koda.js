const fs = require('fs');

class Koda {
    constructor(libraryPath) {
        this.lib = this.loadLibrary(libraryPath);
    }

    loadLibrary(path) {
        return {}; 
    }

    parse(inputString) {
        const resultPtr = this.lib.koda_parse_to_json(inputString);
        const jsonString = this.lib.get_string(resultPtr);
        this.lib.koda_free_string(resultPtr);
        return JSON.parse(jsonString);
    }
}
