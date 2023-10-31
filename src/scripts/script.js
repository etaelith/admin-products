const { invoke } = window.__TAURI__.tauri;
const dialog = window.__TAURI__.dialog;

function displayItems(items) {
    const table = document.querySelector('table tbody');
    table.innerHTML = '';
    items.forEach((item) => {
        const row = table.insertRow();
        const idCell = row.insertCell(0);
        const nameCell = row.insertCell(1);
        const stockCell = row.insertCell(2);
        const priceCell = row.insertCell(3);
        const descriptionCell = row.insertCell(4);
        const editCell = row.insertCell(5);

        idCell.setAttribute('id', 'idDb')
        idCell.textContent = item.id.toString();
        nameCell.textContent = item.name;
        stockCell.textContent = item.stock;
        priceCell.textContent = '$' + item.price.toFixed(2); // Formatear el precio
        descriptionCell.textContent = item.description;

        editCell.classList.add('icons-svg');
        const deleteIcon = document.createElement('img');
        deleteIcon.src = './assets/delete.svg';
        deleteIcon.addEventListener('click', deleteItem)
        const configIcon = document.createElement('img');
        configIcon.src = './assets/config.svg';
        configIcon.addEventListener('click', function () {
            console.log('config')
        })
        editCell.appendChild(deleteIcon);
        editCell.appendChild(configIcon);

    });
}
// get items & show
function getItems() {
    invoke("get_items_db").then((result) => {
        displayItems(result);
    })
}
// delete item by id database
async function deleteItem() {
    const row = this.closest('tr');
    const id = row.querySelector("td")
    const result = await dialog.confirm('Estas seguro que deseas borrar este item?', 'Borrar item')
    if (result) {
        invoke('delete_item_db', {
            id: id.textContent
        })
        getItems()
    } else {
        console.log('nope')
    }
}
async function fitPrice(percent) {
    try {
        await invoke('update_prices_db', {
            percent
        })
        console.log('Precios actualizados exitosamente')
    } catch (error) {
        console.error('Error al actualizar los precios: ', error)
    }

}

// Agree item {name(string),stock(number max u8),price(number max u16),description(string)}
document.querySelector('form').addEventListener('submit', async function (event) {
    event.preventDefault();


    var nameValue = this.elements['name'].value;
    var stockValue = this.elements['stock'].value;
    var priceValue = this.elements['price'].value;
    var descriptionValue = this.elements['description'].value;
    var stockInt = parseInt(stockValue, 10);
    var priceInt = parseInt(priceValue, 10);

    await invoke("save_to_database", {

        name: nameValue,
        stock: stockInt,
        price: priceInt,
        description: descriptionValue
    })
    console.log('Datos guardados en la base de datos local');
    getItems()
});


//reload list
document.getElementById("charge").addEventListener('click', function () {
    getItems()
})

// change value all items n + %
document.getElementById("inflacionSubmit").addEventListener('click', async function () {
    var numero = document.getElementById("inflacion").value;
    var inflacionInt = parseInt(numero, 10);
    const result = await dialog.confirm('Realmente estas seguro de cambiarle a todo un %?', 'Ajuste de precios')
    if (result) {
        fitPrice(inflacionInt)
        getItems()
    } else {
        console.log('nope')
    }
})
