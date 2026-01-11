import { createApp } from 'vue'
import App from './App.vue'
import router from './router'

// Importa o Reset CSS globalmente
import './assets/css/reset.css'

const app = createApp(App)

app.use(router) // Usa o roteador
app.mount('#app')