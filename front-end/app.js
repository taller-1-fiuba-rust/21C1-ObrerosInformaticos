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
                appendRequest(command);
                document.getElementById("command-input").value = "";
                setTimeout(() => {
                    if (request.status == 200){
                        appendResponse(request.responseText);
                    } else if (request.status == 0){
                        console.log(request);
                        appendError("tenes cara de verga");
                    }
                }, 100);
                
            }
        })
    })    
}

function createRequest(command){
    let request = new XMLHttpRequest();
    request.open("POST", "localhost:8080/eval", true);
    request.setRequestHeader('Content-Type', 'application/json');
    request.send(JSON.stringify({
        value: command
    }));
    return request;
}

function appendResponse(response){
    let element = document.createElement("p");
    element.classList += "responseMessage";
    element.innerHTML = response; 
    append(element);
}

function appendError(msg){
    let element = document.createElement("p");
    element.classList += "errorMessage";
    element.innerHTML = "ERROR: " + msg; 
    append(element);
}

function appendRequest(command){
    let text_command = document.createElement("p");
    text_command.style.margin = 0;
    text_command.innerHTML = "<b>></b>  " + command;
    append(text_command);
}

function append(append_item){
    let element = document.getElementById("communication")
    element.appendChild(append_item);
    element.scrollTop = element.scrollHeight;
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