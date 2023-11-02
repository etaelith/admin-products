const { invoke } = window.__TAURI__.tauri
const idProduct = document.getElementById("productId")
const amountProduct = document.getElementById("amount")
const buttonAgree = document.getElementById("agree")
const buttonSell = document.getElementById("sell")
const billingType = document.getElementById("customer")
const tableRows = document.querySelectorAll(".row-item")
const buttonCalculate = document.getElementById("calculate")
const total = document.getElementById("totalAmount")
/*Check exist & stock */
/* idProduct.addEventListener("input", function () {
    const trimmedValue = idProduct.value.trim();
    if (trimmedValue !== "") {
        console.log(trimmedValue)
    }
}) */

/*  Agree item cart fn add_item_table(codebar: i64, amount: i64, usd: i64)*/
buttonAgree.addEventListener("click", function () {
    console.log(Number(idProduct.value), Number(amountProduct.value))
    try {

        invoke("add_item_table", {
            codebar: Number(idProduct.value),
            amount: Number(amountProduct.value),
            usd: 350
        })
    } catch (error) {
        console.error("Error al llamar a add_item_table: ", error)
    }
})

/* Calculate total / fn create_bill(category_type: &str) */

let amountTotal;
buttonCalculate.addEventListener("click", function () {
    let price_total = 0;
    for (let i = 0; i < tableRows.length; i++) {
        const amount = parseFloat(tableRows[i].querySelector("td:nth-child(7)").textContent.replace("$", "").trim());
        price_total = price_total + amount;
    }
    total.innerText = `$ ${price_total}`
    amountTotal = price_total;
    invoke("create_bill", {
        categoryType: billingType.value
    })
})

/* Pagar/ commit sell_completed() */


buttonSell.addEventListener('click', async function () {
    /* buttonCalculate.click();
    console.log("Billing Type: ", billingType.value, " ,Total Amount: ", amountTotal) */
    await invoke("sell_completed");

})