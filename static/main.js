const date_options = {
  weekday: "long",
  day: "numeric",
  month: "long",
  year: "numeric",
  hour: "numeric",
  minute: "numeric",
};
const date_formatter = new Intl.DateTimeFormat("es-ES", date_options);
let interval = null;

$(() => {
  $("#form-invoice").collapse("show");
  $("#send-btn").click(sendBtn);
  $("#payment-btn").click(sendPayment);
  $("#btn-copiar-factura").click(CopyToClipBoard);
});

function insertTd(row, isHtml, value) {
  const cell = document.createElement("td");
  cell[isHtml ? "innerHTML" : "textContent"] = value;
  row.appendChild(cell);
}

const sendBtn = async () => {
  const amount = $("#amount").val();
  const description = $("#description").val();
  $.ajax({
    url: `http://localhost:8000/create_invoice/${description}/${amount}`,
    success: function (invoice) {
      console.log(invoice);
      $("#form-invoice").collapse("hide");
      $("#invoice-amount").text(amount);
      $("#invoice-text").val(invoice.payment_request);
      $("#invoice").collapse("show");
      $("#success-box").collapse("hide");
      new QRCode(document.getElementById("qrcode"), {
        text: invoice.payment_request,
        width: 350,
        height: 350,
        colorDark: "#000000",
        colorLight: "#ffffff",
        correctLevel: QRCode.CorrectLevel.H,
      });
      interval = setInterval(waitPayment, 1000, invoice.hash);
    },
    async: false,
  });
};

const waitPayment = async (hash) => {
  $.ajax({
    url: `http://localhost:8000/invoices/${hash}`,
    success: function (invoice) {
      console.log("INVOICE PAGADO", invoice);
      if (invoice.paid) {
        clearInterval(interval);
        interval = null;
        $("#fechapago").html(
          date_formatter.format(new Date(invoice.settle_date * 1000))
        );
        $("#desc").html(invoice.memo);
        $("#paid-value").html(invoice.value + " sats");
        $("#invoice-state").html(invoice.state == 1 ? "PAGADO" : "CREADO");
        $("#form-invoice").collapse("hide");
        $("#invoice").collapse("hide");
        $("#success-box").collapse("show");
      }
    },
    async: false,
  });
};

async function getBalance() {
  let result = await $.ajax({
    url: "http://localhost:8000/get_wallet_balance",
  });
  return result;
}

async function getListInvoice() {
  let result = await $.ajax({
    url: "http://localhost:8000/invoices",
  });
  return result;
}

function CopyToClipBoard() {
  var copyText = document.getElementById("invoice-text");

  /* Prevent iOS keyboard from opening */
  copyText.readOnly = true;

  /* Change the input's type to text so its text becomes selectable */
  copyText.type = "text";

  /* Select the text field */
  copyText.select();
  copyText.setSelectionRange(0, 99999); /* For mobile devices */

  /* Copy the text inside the text field */
  navigator.clipboard.writeText(copyText.value);

  /* Replace the tooltip's text */
  var tooltip = document.getElementById("myTooltip");
  tooltip.innerHTML = "Copiado";

  /* Change the input's type back to hidden */
  copyText.type = "hidden";
}

const sendPayment = async () => {
  const pr = $("#payment-request").val();
  $.ajax({
    url: `http://localhost:8000/payment`,
    type: "POST",
    data: pr,
    contentType: "text/plain; charset=utf-8",
    dataType: "text",
    success: function (data) {
      const response = JSON.parse(data);
      let message_container = document.getElementById("payment-message");
      message_container.classList.remove("hidden");
      if (response.success) {
        message_container.classList.toggle("alert-success");
        $("#payment-btn").hide();
        $("#home-btn").show();
      } else {
        message_container.classList.toggle("alert-danger");
        if (response.error && response.error.includes("checksum failed")) {
          message_container.innerText = "Factura invalida";
        } else {
          message_container.innerText = response.error;
        }
      }
      console.log(response);
    },
  });
};
