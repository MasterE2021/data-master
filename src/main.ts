// src/main.ts

import {createApp} from "vue";
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'
import Home from './Home.vue'
import VXETable from 'vxe-table'
import 'vxe-table/lib/style.css'

const app = createApp(Home)

app.use(ElementPlus)
app.use(VXETable)
app.mount('#app')