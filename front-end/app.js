function main () {
    executeWhenFind("main-frame", (element) => {
        element.innerHTML = `
            <div id="communication"></div>
            <div id="sender">
                <input name="command"  type="text" id="command-input" >
            </div>`;
    });

    executeWhenFind("command-input", (element)=> {
        element.addEventListener("keydown", (event) => {
            if (event.key === "Enter") {
                let command = document.getElementById("command-input").value; 
                if (command == "" ){
                    return;
                }
                let request = createRequest(command);
                appendIntoCommunication(command);
                document.getElementById("command-input").value = "";
                setTimeout(() => {
                    if (request.status == 200){

                    }
                }, 500);

            }
        })
    })
}

function createRequest(command){
    let request = new XMLHttpRequest();
    request.open("POST", "localhost:8080/", true);
    request.setRequestHeader('Content-Type', 'application/json');
    request.send(JSON.stringify({
        value: command
    }));
    return request;
}

function appendIntoCommunication(command){
    let text_command = document.createElement("p");
    text_command.style.margin = 0;
    text_command.innerHTML = command;
    document.getElementById("communication").appendChild(text_command);
}

function executeWhenFind(id, apply, duration, begin) {
    var node = document.getElementById(id);
    
    if (node == null) {
        if (begin == null) {
            if (duration == null) { duration = 6000; } //intenta durante 60 segundos
            begin = new Date().getTime();
        }
        if (new Date().getTime() - begin < duration) {
            setTimeout(function() { executeWhenFind(id, apply, duration, begin); }, 100);
        }
    }
    else {
        apply(node);
    }
}

main();