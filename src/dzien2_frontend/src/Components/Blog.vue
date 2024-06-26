<template>
    <div>
        
        <h1 class="text-blue-600">Wpisy na bloga:</h1>
        <div class="w-100 flex flex-row-reverse">
            <button @click="pobierzWpisy" class="text-green-200 bg-blue-600 rounded-md p-4">Pobierz</button>
        </div>
        
        <div class="grid mx-6 gap-4 my-4">
            <div v-for="wpis in wpisy"
            class="drop-shadow-x1 bg-stone-300 p-4">
                <p>{{ wpis }}</p>
            </div>
        </div>

        <div class="flex justify-center">
        <input v-model="nowyBlog" type="text"
            class="border-2 border-blue-600 p-4 rounded-md w-100">
            
            <button @click="dodajWpis" class="text-green-200 bg-blue-600 rounded-md p-4 ml-4">Dodaj</button>
        </div>
    </div>
</template>

<script>
import { dzien2_backend } from 'declarations/dzien2_backend/index';

export default {
    data()
    {
        return {
            wpisy: [],
            nowyBlog: ""
        }
    },

    methods: {
        async dodajWpis()
        {
            await dzien2_backend.dodaj_wpis(this.nowyBlog);
            this.pobierzWpisy();
        },

        async pobierzWpisy()
        {
            this.wpisy = await dzien2_backend.odczytaj_wpisy();
        }
    },
    async mounted() {
        this.pobierzWpisy();
    }
}

</script>