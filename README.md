# LNTIP
LNTIP es un proyecto que te permite enviar y recibir pagos a través de la red Lightning de Bitcoin de manera sencilla y segura.
Es una version extendia del repositorio

[Francisco Calderón (@grunch) - Rust Lightning Workshop](https://github.com/grunch/rust-lightning-workshop)


# ¿Qué es Lightning Network?
Lightning Network es una capa adicional a la red de Bitcoin que permite realizar transacciones de manera rápida y económica. Utiliza una red de canales de pago fuera de la cadena de bloques de Bitcoin para permitir transacciones rápidas y bajas comisiones.

# Características principales de LNTIP
* Interfaz de usuario amigable para enviar y recibir pagos.
* Integración con el monedero LND (Lightning Network Daemon) para acceder a la red Lightning de manera segura.
* Posibilidad de generar y escanear invoices (facturas) en formato QR.
* Historial de transacciones para ver el estado de tus pagos.

# Instalación y uso
Para instalar y utilizar LNTIP sigue estos pasos:

* Descarga el código fuente de LNTIP desde este repositorio.
* Instala las dependencias necesarias ejecutando ```cargo install.```
* Configura LNTIP editando el archivo .env con tus datos de conexión a la red Lightning Network. En este paso puedes utilizar [Polar](https://github.com/jamaljsr/polar)
* Inicia LNTIP ejecutando ```cargo run```.
* Abre LNTIP en tu navegador web utilizando la dirección ```http://localhost:8000```

# Licencia
LNTIP se distribuye bajo la licencia MIT. Consulta el archivo LICENSE para obtener más información.

# Contribución
Si deseas contribuir al proyecto LNTIP, sigue estos pasos:

* Clona el repositorio de LNTIP en tu computadora: ```git clone https://github.com/eimolina/lntip-rust.git```
* Crea una rama para tu contribución: ```git checkout -b mi-contribucion```
* Realiza tus cambios y commitealos: ```git commit -am "Añadir alguna nueva funcionalidad"```
* Envía tus cambios a tu rama: ```git push origin mi-contribucion```
* Crea un pull request para que tu contribución sea revisada y fusionada al proyecto principal