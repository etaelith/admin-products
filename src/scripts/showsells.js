const { invoke } = window.__TAURI__.tauri;
function showItems(items) {
    const table = document.getElementById('tbody_items');
    table.innerHTML = '';
    items.forEach((item) => {
        const row = table.insertRow();
        const idCell = row.insertCell(0)
        const idProduct = row.insertCell(1)
        const amountCell = row.insertCell(2)
        const price_arsCell = row.insertCell(3)
        const usd_valueCell = row.insertCell(4)
        const totalCell = row.insertCell(5)

        idCell.textContent = item.id.toString();
        idProduct.textContent = item.codebar.toString();
        amountCell.textContent = item.amount
        price_arsCell.textContent = item.price
        usd_valueCell.textContent = item.usd_value
        totalCell.textContent = item.total

    })
}
function showBuyers(buyers) {
    const table = document.getElementById('tbody_buyers');
    table.innerHTML = '';
    buyers.forEach((buyer) => {
        const row = table.insertRow();
        const idCell = row.insertCell(0)
        const categoryCell = row.insertCell(1)
        const dniCell = row.insertCell(2)
        const price_usdCell = row.insertCell(3)
        const price_arsCell = row.insertCell(4)
        idCell.textContent = buyer.id.toString();
        categoryCell.textContent = buyer.category_type;
        dniCell.textContent = buyer.dni
        price_usdCell.textContent = buyer.total_usd
        price_arsCell.textContent = buyer.total
    })
}

function showItemsSell() {
    invoke("get_items_sell").then((result) => {
        showItems(result)
    })
}

function showBuyersSell() {
    invoke("get_buyers").then((result) => {
        showBuyers(result)
    })
}
document.getElementById('reload_sells').addEventListener('click', () => {
    showItemsSell()
})

document.getElementById('reload_buyers').addEventListener('click', () => {
    showBuyersSell()
})