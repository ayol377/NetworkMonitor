function map(){
    var nodes = [];
    var edges = [];
    invoke('mapdata').then((devs) => {
        var gateway = "";
        for (i in devs){
            var node = {id: 'x', label: 'y'};
            var edge = {from: 'x', to: 'y'};

            var dev = devs[i];
            node.id = dev[1];
            if (dev[0] == "UNKNOWN"){
                node.label = dev[2];
            }else{
                node.label = dev[0];
            }
            nodes.push(node);

            if (i == 0){
                gateway = dev[1];
            }else{
                edge.from = gateway;
                edge.to = dev[1];
                edges.push(edge);
            }

        }
        console.log(nodes);
        console.log(edges);
        var nodes1 = new vis.DataSet(nodes);
        var edges1 = new vis.DataSet(edges);
        var container = document.getElementById("network-div");
        var data = {nodes: nodes1, edges: edges1};
        var options = {
            clickToUse: false,
            interaction:{
                dragNodes:false,
                dragView: true,
                hideEdgesOnDrag: false,
                hideEdgesOnZoom: false,
                hideNodesOnDrag: false,
                hover: false,
                hoverConnectedEdges: true,
                keyboard: {
                  enabled: false,
                  speed: {x: 10, y: 10, zoom: 0.02},
                  bindToWindow: true,
                  autoFocus: true,
                },
                multiselect: false,
                navigationButtons: false,
                selectable: false,
                selectConnectedEdges: true,
                tooltipDelay: 300,
                zoomSpeed: 1,
                zoomView: true
              }
        };
        var network = new vis.Network(container, data, options);
    });
}

map();