<script setup>
import { ref, onMounted, computed, nextTick } from 'vue'
import { useRouter } from 'vue-router'
import Chart from 'chart.js/auto'

const router = useRouter()
const API_URL = "http://127.0.0.1:46000"

// ESTADO
const nomeUsuario = ref('Visitante')
const saldoTotal = ref(0)
const listaGlobalTransacoes = ref([])
const dataAtual = ref(new Date())

// Modais
const modalAberto = ref(false) // Modal de Nova Transação
const modalGraficoAberto = ref(false) // Modal do Gráfico

const tipoPersonalizadoVisivel = ref(false)
// campo 'naturezaCustom' (padrão é saida)
const form = ref({
  valor: '',
  tipo: 'lazer',
  customTipo: '',
  naturezaCustom: 'saida' 
})

// Filtros
const filtroAtivo = ref('tudo')

// ESTADO DO GRÁFICO 
const graficoCanvas = ref(null)
let graficoInstance = null
const modoGrafico = ref('pizza') // pizza ou linha
const anoSelecionado = ref(new Date().getFullYear())

// Gera os últimos 3 anos para o select
const anosDisponiveis = computed(() => {
  const anoAtual = new Date().getFullYear()
  return [anoAtual, anoAtual - 1, anoAtual - 2]
})

// COMPUTADOS
const nomeMesAno = computed(() => {
  return dataAtual.value.toLocaleDateString('pt-BR', { month: 'long', year: 'numeric' }).toUpperCase()
})

const saldoFormatado = computed(() => {
  return saldoTotal.value.toLocaleString('pt-BR', { style: 'currency', currency: 'BRL' })
})

// Lógica das "Pastas" (Categorias)

// Descobre quais categorias existem na lista atual (ex: 'lazer', 'mercado')
const categoriasDisponiveis = computed(() => {
  const todas = listaGlobalTransacoes.value.map(t => t.tipo)
  return [...new Set(todas)] // O Set remove nomes repetidos
})

// Filtra a lista baseada no botão clicado
const transacoesFiltradas = computed(() => {
  let lista = listaGlobalTransacoes.value
  const filtro = filtroAtivo.value

  // Lógica de Filtragem Atualizada
  if (filtro === 'tudo') {
    // Não faz nada, mostra a lista inteira
  } else if (filtro === 'entradas') {
    lista = lista.filter(t => t.valor > 0)
  } else if (filtro === 'saídas') {
    lista = lista.filter(t => t.valor < 0)
  } else {
    // SE NÃO FOR entrada/saida/tudo, ENTÃO É UMA CATEGORIA (PASTA)
    lista = lista.filter(t => t.tipo === filtro)
  }

  // Mantém a ordenação por data (mais recente primeiro)
  return [...lista].sort((a, b) => new Date(b.data) - new Date(a.data))
})

// Função para o HTML usar quando clicar no botão
function definirFiltro(novoFiltro) {""
  filtroAtivo.value = novoFiltro
}

// MÉTODOS
onMounted(() => {
  const user = localStorage.getItem("userName")
  if (!user) {
    router.push('/login')
  } else {
    nomeUsuario.value = user
    carregarTransacoes()
  }
})

function mudarMes(delta) {
  const novaData = new Date(dataAtual.value)
  novaData.setMonth(novaData.getMonth() + delta)
  dataAtual.value = novaData
  carregarTransacoes()
}

async function carregarTransacoes() {
  try {
    const mes = dataAtual.value.getMonth() + 1
    const ano = dataAtual.value.getFullYear()

    const res = await fetch(`${API_URL}/transactions/by_date/${mes}/${ano}`)

    if (res.ok) {
      listaGlobalTransacoes.value = await res.json()
    } else if (res.status === 404) {
      listaGlobalTransacoes.value = []
    }

    calcularSaldo()

  } catch (error) {
    console.error("Erro API:", error)
    listaGlobalTransacoes.value = []
    calcularSaldo()
  }
}

function calcularSaldo() {
  saldoTotal.value = listaGlobalTransacoes.value.reduce((acc, item) => acc + item.valor, 0)
}

// FUNÇÃO SALVAR 
async function salvarTransacao() {
  let tipoFinal = form.value.tipo
  
  // Define se é Saída ou Entrada
  let ehDespesa = true // Assume que é despesa por padrão

  if (tipoFinal === 'entrada') {
    ehDespesa = false // Se for a opção padrão "Entrada", não é despesa
  } else if (tipoFinal === 'outro') {
    // Se for Personalizado, pega o nome digitado E a escolha do usuário
    tipoFinal = form.value.customTipo.trim()
    if (!tipoFinal) return alert("Digite o nome da categoria!")
    
    // Verifica o Radio Button (Se marcou Entrada, não é despesa)
    if (form.value.naturezaCustom === 'entrada') {
        ehDespesa = false
    }
  }

  // Prepara o valor (Negativo se for despesa, Positivo se for entrada)
  let valorFinal = Math.abs(parseFloat(form.value.valor))
  if (ehDespesa) {
    valorFinal = valorFinal * -1
  }

  const payload = { tipo: tipoFinal, valor: valorFinal }

  try {
    const res = await fetch(`${API_URL}/add_transaction`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(payload)
    })

    if (res.ok) {
      modalAberto.value = false
      // Limpa o formulário
      form.value.valor = ''
      form.value.tipo = 'entrada'
      form.value.customTipo = ''
      form.value.naturezaCustom = 'saida' // Reseta para saida
      tipoPersonalizadoVisivel.value = false
      
      carregarTransacoes()
    } else {
      alert("Erro ao salvar")
    }
  } catch (e) {
    alert("Erro de conexão")
  }
}

async function excluir(id) {
  if (!confirm("Excluir item?")) return
  try {
    const res = await fetch(`${API_URL}/delete_transaction/${id}`, { method: 'DELETE' })
    if (res.ok) carregarTransacoes()
  } catch (e) {
    alert("Erro ao excluir")
  }
}

// LÓGICA DO GRÁFICO
async function abrirGrafico() {
    modalGraficoAberto.value = true;
    await nextTick();
    atualizarGrafico(); // Função centralizadora
}

async function atualizarGrafico() {
  if (!graficoCanvas.value) return
  
  // Limpa gráfico anterior
  if (graficoInstance) {
    graficoInstance.destroy()
    graficoInstance = null
  }

  if (modoGrafico.value === 'pizza') {
    desenharGraficoPizza()
  } else {
    await desenharGraficoLinha()
  }
}

function desenharGraficoPizza() {
  const lista = listaGlobalTransacoes.value 
  const filtro = filtroAtivo.value

  let labels = []
  let data = []
  let colors = []
  let tituloGrafico = '' // Variável para controlar o título

  // CASO 1: Visão Geral (Tudo, Entradas ou Saídas)
  if (['tudo', 'entradas', 'saídas'].includes(filtro)) {
      const entradas = lista.filter(t => t.valor > 0).reduce((acc, t) => acc + t.valor, 0)
      const saidas = lista.filter(t => t.valor < 0).reduce((acc, t) => acc + (t.valor * -1), 0)
      
      labels = ['Total Entradas', 'Total Saídas']
      data = [entradas, saidas]
      colors = ['#10b981', '#ef4444']
      
      // AQUI O AJUSTE: Mesmo que o filtro seja 'entradas', o gráfico mostra TUDO.
      tituloGrafico = 'VISÃO GERAL (TUDO)' 
  } 
  // CASO 2: Visão da PASTA ESPECÍFICA
  else {
      const itensDaPasta = lista.filter(t => t.tipo === filtro)

      const entradasPasta = itensDaPasta
          .filter(t => t.valor > 0)
          .reduce((acc, t) => acc + t.valor, 0)

      const saidasPasta = itensDaPasta
          .filter(t => t.valor < 0)
          .reduce((acc, t) => acc + (t.valor * -1), 0)

      labels = [`Entradas (${filtro})`, `Saídas (${filtro})`]
      data = [entradasPasta, saidasPasta]
      colors = ['#10b981', '#ef4444']
      
      // Aqui mantém o nome da pasta (ex: LAZER, FARMACIA)
      tituloGrafico = filtro.toUpperCase()
  }

  graficoInstance = new Chart(graficoCanvas.value, {
    type: 'doughnut',
    data: {
      labels: labels,
      datasets: [{
        data: data,
        backgroundColor: colors,
        borderWidth: 0,
        hoverOffset: 4
      }]
    },
    options: {
      responsive: true,
      maintainAspectRatio: false,
      plugins: { 
          legend: { position: 'bottom' },
          title: { 
            display: true, 
            text: tituloGrafico, // Usa a variável que definimos acima
            font: { size: 16 }
          }
      }
    }
  })
}

// Função auxiliar para pegar dados do ano todo (loopando a API)
async function buscarDadosDoAno(ano) {
  const promises = []
  // Busca os 12 meses em paralelo
  for(let i=1; i<=12; i++) {
     promises.push(fetch(`${API_URL}/transactions/by_date/${i}/${ano}`).then(r => r.ok ? r.json() : []))
  }
  const resultados = await Promise.all(promises)
  
  // Junta tudo num array só
  let todasTransacoes = []
  resultados.forEach(listaMes => todasTransacoes.push(...listaMes))
  return todasTransacoes
}

async function desenharGraficoLinha() {
  const dadosAno = await buscarDadosDoAno(anoSelecionado.value)
  const filtro = filtroAtivo.value
  
  const meses = ['Jan', 'Fev', 'Mar', 'Abr', 'Mai', 'Jun', 'Jul', 'Ago', 'Set', 'Out', 'Nov', 'Dez']
  const valoresEntrada = new Array(12).fill(0)
  const valoresSaida = new Array(12).fill(0)

  // Define se é filtro geral ou categoria específica
  const ehFiltroGeral = ['tudo', 'entradas', 'saídas'].includes(filtro)
  
  //  Define o Título correto
  let tituloGrafico = ''
  if (ehFiltroGeral) {
      tituloGrafico = 'VISÃO GERAL (TUDO)'
  } else {
      tituloGrafico = `Evolução: ${filtro.toUpperCase()}`
  }

  // Processa os dados
  dadosAno.forEach(t => {
    // Se for Geral, pega tudo. Se for Pasta, só pega se o tipo bater.
    if (ehFiltroGeral || t.tipo === filtro) {
        
        const dataT = new Date(t.data)
        if (!isNaN(dataT)) {
            const mesIndex = dataT.getMonth() // 0 a 11
            
            if (t.valor > 0) {
                valoresEntrada[mesIndex] += t.valor
            } else {
                valoresSaida[mesIndex] += (t.valor * -1)
            }
        }
    }
  })

  // Monta as duas linhas (Sempre Verde e Vermelho)
  const datasets = [
    { 
        label: 'Entradas', 
        data: valoresEntrada, 
        borderColor: '#10b981', 
        backgroundColor: '#10b981', 
        tension: 0.3 
    },
    { 
        label: 'Saídas', 
        data: valoresSaida, 
        borderColor: '#ef4444', 
        backgroundColor: '#ef4444', 
        tension: 0.3 
    }
  ]

  graficoInstance = new Chart(graficoCanvas.value, {
    type: 'line',
    data: { labels: meses, datasets: datasets },
    options: {
      responsive: true,
      maintainAspectRatio: false,
      scales: { y: { beginAtZero: true } },
      plugins: { 
          title: { 
              display: true, 
              text: tituloGrafico, // <--- Título corrigido aqui
              font: { size: 16 }
          } 
      }
    }
  })
}

function logout() {
  localStorage.removeItem("userName")
  router.push('/login')
}

function verificarTipoCustom() {
  tipoPersonalizadoVisivel.value = (form.value.tipo === 'outro')
}

const formatarData = (dataStr) => new Date(dataStr).toLocaleDateString('pt-BR', { day: '2-digit', month: '2-digit' })
</script>

<template>
  <div class="dashboard-body">
    <header>
        <div id="upper">
            <div id="logo">
                <img src="/images/Piggy - Copia.png" alt="LogoPiggy">
                <p id="Piggy">PiggyBank</p>
            </div>
            <div id="saudation">
                <p>Saudações, <span>{{ nomeUsuario }}</span>!</p>
                <button id="btnSair" @click="logout">Sair</button>
            </div>
        </div>
    </header>

    <main class="dashboard-container">
        <section id="area-saldo">
            <p>Saldo Total</p>
            <h1>{{ saldoFormatado }}</h1>

            <button class="btn-ver-grafico" @click="abrirGrafico">
                📊 Ver Gráfico
            </button>
        </section>

        <div class="controles-topo">
            <div class="seletor-data">
                <button @click="mudarMes(-1)">&lt;</button>
                <span id="data-atual">{{ nomeMesAno }}</span>
                <button @click="mudarMes(1)">&gt;</button>
            </div>
            <button id="btn-nova-transacao" @click="modalAberto = true">+ NOVA</button>
        </div>

        <div class="filtros-scroll">
          <button 
              class="filtro-chip" 
              :class="{ ativo: filtroAtivo === 'tudo' }" 
              @click="definirFiltro('tudo')">
              🏠 Tudo
          </button>
          <button 
              class="filtro-chip" 
              :class="{ ativo: filtroAtivo === 'entradas' }" 
              @click="definirFiltro('entradas')">
              ⬇ Entradas
          </button>
          <button 
              class="filtro-chip" 
              :class="{ ativo: filtroAtivo === 'saídas' }" 
              @click="definirFiltro('saídas')">
              ⬆ Saídas
          </button>
          
          <div class="divisor-vertical"></div>

          <button 
              v-for="cat in categoriasDisponiveis" 
              :key="cat"
              class="filtro-chip capitalize" 
              :class="{ ativo: filtroAtivo === cat }" 
              @click="definirFiltro(cat)">
              {{ cat }}
          </button>
      </div>

        <ul id="lista-transacoes">
            <li v-for="item in transacoesFiltradas" :key="item.id" class="item-transacao">
                <div class="info-esquerda">
                    <div class="icone-tipo" :class="item.valor > 0 ? 'entrada' : 'saida'">
                        {{ item.valor > 0 ? '⬇' : '⬆' }}
                    </div>
                    <div class="detalhes">
                        <strong class="titulo">{{ item.tipo.toUpperCase() }}</strong>
                        <span class="data">{{ " "+formatarData(item.data) }}</span>
                    </div>
                </div>
                <div class="info-direita">
                    <strong class="valor" :class="item.valor > 0 ? 'positivo' : 'negativo'">
                        {{ item.valor.toLocaleString('pt-BR', { style: 'currency', currency: 'BRL' }) }}
                    </strong>
                    <div class="acoes">
                        <button class="btn-excluir" @click="excluir(item.id)">🗑️</button>
                    </div>
                </div>
            </li>
        </ul>
    </main>

    <div id="modal-overlay" v-if="modalAberto" @click.self="modalAberto = false">
        <div class="modal-box">
            <h2>Nova Transação</h2>
            <form @submit.prevent="salvarTransacao">
                <div class="campo">
                    <label>Valor (R$)</label>
                    <input type="number" step="0.01" required v-model="form.valor" placeholder="0,00">
                </div>
                <div class="campo">
                  <label>Tipo</label>
                  <select v-model="form.tipo" @change="verificarTipoCustom" required>
                      <option value="lazer">🔴 Lazer</option>
                      <option value="supermercado">🔴 Supermercado</option>
                      <option value="outro">✨ Personalizado...</option>
                  </select>
                  
                  <div v-if="tipoPersonalizadoVisivel" class="personalizado-wrapper">
                      <input type="text" v-model="form.customTipo" placeholder="Nome (ex: Freela, Venda de Bolo)..." class="input-custom">
                      
                      <div class="opcao-natureza">
                          <label class="radio-btn">
                              <input type="radio" value="entrada" v-model="form.naturezaCustom">
                              <span>💰 Ganho</span>
                          </label>
                          <label class="radio-btn">
                              <input type="radio" value="saida" v-model="form.naturezaCustom">
                              <span>💸 Gasto</span>
                          </label>
                      </div>
                  </div>
                </div>
                <div class="botoes-modal">
                    <button type="button" id="btn-cancelar" @click="modalAberto = false">Cancelar</button>
                    <button type="submit" id="btn-salvar">Salvar</button>
                </div>
            </form>
        </div>
    </div>

    <div id="modal-overlay" v-if="modalGraficoAberto" @click.self="modalGraficoAberto = false">
        <div class="modal-box modal-grafico-box">
            <div class="modal-header">
                <h2>Estatísticas</h2>
                <button class="btn-fechar-x" @click="modalGraficoAberto = false">✕</button>
            </div>

            <div class="grafico-controles">
                <div class="toggle-group">
                    <button 
                        :class="{ ativo: modoGrafico === 'pizza' }" 
                        @click="modoGrafico = 'pizza'; atualizarGrafico()"
                    >Pizza (Mês)</button>
                    <button 
                        :class="{ ativo: modoGrafico === 'linha' }" 
                        @click="modoGrafico = 'linha'; atualizarGrafico()"
                    >Linha (Ano)</button>
                </div>

                <select 
                    v-if="modoGrafico === 'linha'" 
                    v-model="anoSelecionado" 
                    @change="atualizarGrafico"
                    class="select-ano"
                >
                    <option v-for="ano in anosDisponiveis" :key="ano" :value="ano">{{ ano }}</option>
                </select>
            </div>

            <div class="grafico-container-modal">
                <canvas ref="graficoCanvas"></canvas>
            </div>
        </div>
    </div>

  </div>
</template>

<style scoped>

.dashboard-body {
    background-color: #11034b86;
    min-height: 100vh;
    width: 100%;
    font-family: Arial, sans-serif;
    display: flex;
    flex-direction: column;
}

header {
    width: 100%;
    background-color: #d3d3d3;
    box-shadow: 0 4px 12.1px rgba(0, 0, 0, 0.5);
    z-index: 10;
}

#upper {
    display: flex;
    align-items: center;
    justify-content: space-between;
    height: 56px;
    padding: 0 20px;
    max-width: 1200px;
    margin: 0 auto;
}

#logo {
    display: flex;
    align-items: center;
    gap: 10px;
}
#logo img { width: 32px; height: 32px; }
#Piggy { font-size: 24px; color: black; margin: 0; font-weight: bold; }

#saudation {
    display: flex;
    align-items: center;
    font-size: 18px;
    gap: 15px;
}

#btnSair {
    height: 35px;
    padding: 0 20px;
    background: linear-gradient(45deg, #ff85d8, #FFAAEA);
    color: white;
    border: none;
    border-radius: 10px;
    font-weight: bold;
    cursor: pointer;
    box-shadow: 0 2px 5px rgba(0,0,0,0.2);
}

.dashboard-container {
    width: 100%;
    max-width: 600px;
    margin: 20px auto;
    padding: 0 20px;
    flex: 1;
}

#area-saldo {
    text-align: center;
    background: linear-gradient(135deg, #FFAAEA, #ff85d8);
    padding: 30px;
    border-radius: 20px;
    color: white;
    box-shadow: 0 10px 20px rgba(255, 105, 180, 0.3);
    margin-bottom: 25px;
}
#area-saldo h1 { font-size: 42px; margin-top: 10px; }

/* Estilo novo do Botão Ver Gráfico */
.btn-ver-grafico {
    margin-top: 15px;
    background-color: rgba(255, 255, 255, 0.2);
    border: 1px solid white;
    color: rgb(0, 26, 255);
    padding: 8px 16px;
    border-radius: 20px;
    cursor: pointer;
    transition: 0.2s;
    font-weight: bold;
}
.btn-ver-grafico:hover {
    background-color: white;
    color: #ff85d8;
}

.controles-topo {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 20px;
}
.seletor-data {
    display: flex;
    align-items: center;
    gap: 10px;
    font-weight: bold;
    color: white;
}
.seletor-data button {
    width: 30px; height: 30px;
    border-radius: 50%;
    border: 1px solid #ccc;
    background: white;
    cursor: pointer;
}

#btn-nova-transacao {
    background-color: #333;
    color: white;
    border: none;
    padding: 10px 20px;
    border-radius: 20px;
    font-weight: bold;
    cursor: pointer;
}

.filtros {
    display: flex;
    gap: 10px;
    margin-bottom: 20px;
}
.filtro-btn {
    background-color: white;
    border: 1px solid #eee;
    padding: 8px 16px;
    border-radius: 20px;
    cursor: pointer;
    color: #666;
}
.filtro-btn.ativo {
    background-color: #FFAAEA;
    color: white;
    border-color: #FFAAEA;
}

#lista-transacoes { list-style: none; padding: 0; }
.item-transacao {
    background-color: white;
    padding: 15px;
    margin-bottom: 12px;
    border-radius: 15px;
    display: flex;
    justify-content: space-between;
    align-items: center;
    box-shadow: 0 4px 10px rgba(0,0,0,0.03);
}

.info-esquerda { display: flex; align-items: center; gap: 15px; }
.icone-tipo {
    width: 40px; height: 40px; border-radius: 10px;
    display: flex; align-items: center; justify-content: center; font-size: 20px;
}
.icone-tipo.entrada { background-color: #d1fae5; color: #065f46; }
.icone-tipo.saida { background-color: #fee2e2; color: #991b1b; }

.info-direita { text-align: right; }
.valor.positivo { color: #10b981; }
.valor.negativo { color: #ef4444; }

.btn-excluir {
    background: none; border: none; cursor: pointer; font-size: 14px; opacity: 0.5;
}
.btn-excluir:hover { opacity: 1; }

#modal-overlay {
    position: fixed; top: 0; left: 0;
    width: 100%; height: 100%;
    background-color: rgba(0, 0, 0, 0.5);
    display: flex; justify-content: center; align-items: center;
    z-index: 1000;
}
.modal-box {
    background: white; padding: 25px; border-radius: 15px; width: 90%; max-width: 400px;
}

/* Estilos para o Modal do Gráfico */
.modal-grafico-box {
    max-width: 500px;
    text-align: center;
}
.modal-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 15px;
}
.btn-fechar-x {
    background: none;
    border: none;
    font-size: 20px;
    cursor: pointer;
    color: #666;
}
.grafico-container-modal {
    position: relative;
    height: 300px;
    width: 100%;
}

/* --- ESTILOS NOVOS DOS CONTROLES DO GRÁFICO --- */
.grafico-controles {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 15px;
    background-color: #f5f5f5;
    padding: 5px;
    border-radius: 10px;
}

.toggle-group {
    display: flex;
    gap: 5px;
}

.toggle-group button {
    border: none;
    background: transparent;
    padding: 6px 12px;
    border-radius: 6px;
    cursor: pointer;
    font-size: 14px;
    color: #666;
    transition: 0.2s;
}

.toggle-group button.ativo {
    background-color: white;
    color: #ff85d8;
    font-weight: bold;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
}

.select-ano {
    padding: 5px 10px;
    border-radius: 6px;
    border: 1px solid #ddd;
    background: white;
    cursor: pointer;
}

.campo { margin-bottom: 15px; display: flex; flex-direction: column; }
.campo input, .campo select { padding: 10px; border: 1px solid #ddd; border-radius: 8px; }
.botoes-modal { display: flex; gap: 10px; margin-top: 20px; }
#btn-salvar { flex: 1; background-color: #FFAAEA; color: white; border: none; padding: 12px; border-radius: 8px; font-weight: bold; cursor: pointer; }
#btn-cancelar { flex: 1; background-color: transparent; border: 1px solid #ccc; color: #666; padding: 12px; border-radius: 8px; cursor: pointer; }

/*BARRA DE FILTROS/PASTAS*/

.filtros-scroll {
    display: flex;
    gap: 10px;
    margin-bottom: 20px;
    overflow-x: auto; /* Permite rolar para o lado se tiver muitas categorias */
    padding-bottom: 5px; /* Espaço para a barra de rolagem não colar */
    white-space: nowrap; /* Impede que os botões quebrem linha */
    
    /* Esconde a barra de rolagem feia (Opcional) */
    scrollbar-width: thin; 
    -ms-overflow-style: none; 
}
.filtros-scroll::-webkit-scrollbar {
    height: 4px; /* Barra bem fininha */
}
.filtros-scroll::-webkit-scrollbar-thumb {
    background-color: rgba(255, 255, 255, 0.3);
    border-radius: 4px;
}

.filtro-chip {
    background-color: white;
    border: 1px solid #eee;
    padding: 8px 16px;
    border-radius: 20px;
    cursor: pointer;
    color: #666;
    font-size: 14px;
    transition: 0.2s;
    flex-shrink: 0; /* Garante que o botão não seja esmagado */
}

.filtro-chip:hover {
    background-color: #f0f0f0;
}

.filtro-chip.ativo {
    background-color: #FFAAEA;
    color: white;
    border-color: #FFAAEA;
    font-weight: bold;
    box-shadow: 0 4px 6px rgba(255, 105, 180, 0.2);
}

.divisor-vertical {
    width: 1px;
    background-color: rgba(255,255,255,0.5);
    margin: 0 5px;
}

.capitalize {
    text-transform: capitalize; /* Deixa a primeira letra maiúscula (lazer -> Lazer) */
}
.personalizado-wrapper {
    margin-top: 10px;
    background-color: #f9f9f9;
    padding: 10px;
    border-radius: 8px;
    border: 1px solid #eee;
}

.input-custom {
    width: 100%;
    margin-bottom: 10px;
    box-sizing: border-box; /* Garante que não estoure a largura */
}

.opcao-natureza {
    display: flex;
    gap: 15px;
    justify-content: center;
}

.radio-btn {
    display: flex;
    align-items: center;
    gap: 5px;
    cursor: pointer;
    font-weight: bold;
    font-size: 14px;
}

.radio-btn input {
    accent-color: #ff85d8; /* Cor da bolinha quando marcada */
    width: 18px;
    height: 18px;
}
</style>