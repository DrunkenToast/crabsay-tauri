import { ref } from "vue";

class Settings {
    text = ref("");
    color = ref("#000");

    constructor() {
        fetch('https://whatthecommit.com/index.txt')
            .then((res) => res.text())
            .then((data) => {
                this.text.value = data.trim();
            });
    }
}

export default new Settings();
