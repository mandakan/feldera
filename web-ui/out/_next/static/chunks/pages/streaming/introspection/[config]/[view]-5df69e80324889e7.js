(self.webpackChunk_N_E=self.webpackChunk_N_E||[]).push([[550],{9856:function(e,t,n){(window.__NEXT_P=window.__NEXT_P||[]).push(["/streaming/introspection/[config]/[view]",function(){return n(4289)}])},9541:function(e,t,n){"use strict";var i=n(5893),o=n(6886);let r=e=>{let{title:t,subtitle:n}=e;return(0,i.jsxs)(o.ZP,{item:!0,xs:12,children:[t,n]})};t.Z=r},4289:function(e,t,n){"use strict";n.r(t);var i=n(5893),o=n(6242),r=n(6886),s=n(5861),a=n(747),l=n(7987),d=n(7848),u=n(1163),c=n(7294),f=n(9541),h=n(7838),g=n(8820),p=n(3293);let m=()=>{let e=(0,a.A)(),[t,n]=(0,c.useState)(void 0),[m,w]=(0,c.useState)(void 0),[v,_]=(0,c.useState)(void 0),[E,j]=(0,c.useState)(void 0),x=(0,u.useRouter)(),{config:N,view:I}=x.query,S=(0,d.a)(["projectStatus",{project_id:null==t?void 0:t.project_id}],{enabled:void 0!==t&&void 0!==t.project_id});(0,c.useEffect)(()=>{if(!S.isLoading&&!S.isError&&E&&S.data&&S.data.schema){let e=(0,p.S)(S.data),t=e.schema.outputs.find(e=>e.name===E);t&&w([{field:"genId",headerName:"genId",flex:.1}].concat(t.fields.map(e=>({field:e.name,headerName:e.name,flex:1}))).concat([{field:"weight",headerName:"weight",flex:.2}]))}},[S.isLoading,S.isError,S.data,w,E]),(0,c.useEffect)(()=>{"string"==typeof N&&parseInt(N)!=v&&_(parseInt(N)),"string"==typeof I&&j(I)},[v,_,N,I,j]);let b=(0,d.a)(["configStatus",{config_id:v}],{enabled:void 0!==v});(0,c.useEffect)(()=>{b.isLoading||b.isError||n(b.data)},[b.isLoading,b.isError,b.data,n]);let Z=(0,c.useRef)(null);return(0,c.useEffect)(()=>{if(t&&t.pipeline&&void 0!==I&&void 0!==m&&e.current){let n=t.attached_connectors.find(e=>e.config==I),i=(null==n?void 0:n.direction)===h.Nm.INPUT?"/input_endpoint/":"/output_endpoint/",o=new WebSocket("ws://localhost:"+t.pipeline.port+i+"debug-"+(null==n?void 0:n.uuid));return o.onopen=()=>{console.log("opened")},o.onclose=()=>{console.log("closed")},o.onmessage=t=>{t.data.text().then(t=>{(0,g.Qc)(t,{delimiter:","},(t,n)=>{var i;t&&console.error(t);let o=n.map(e=>{let t=e[0],n=e[e.length-1],i=e.slice(0,e.length-1),o={genId:t,weight:parseInt(n)};return m.forEach((e,t)=>{"genId"!==e.field&&"weight"!==e.field&&(o[e.field]=i[t-1])}),o});null===(i=e.current)||void 0===i||i.updateRows(o.map(t=>{let n=e.current.getRow(t.genId);return null!==n&&n.weight+t.weight==0?t:null==n&&t.weight<0?null:{...t,weight:t.weight+((null==n?void 0:n.weight)||0)}}).filter(e=>null!==e))})})},Z.current=o,()=>{o.close()}}},[t,I,e,m]),!b.isLoading&&!b.isError&&m&&(0,i.jsxs)(r.ZP,{container:!0,spacing:6,className:"match-height",children:[(0,i.jsx)(f.Z,{title:(0,i.jsxs)(s.Z,{variant:"h5",children:[" ",null==t?void 0:t.name," / ",E]}),subtitle:(0,i.jsx)(s.Z,{variant:"body2",children:"Introspection"})}),(0,i.jsx)(r.ZP,{item:!0,xs:12,children:(0,i.jsx)(o.Z,{children:(0,i.jsx)(l.s,{getRowId:e=>e.genId,apiRef:e,autoHeight:!0,columns:m,rowThreshold:0,rows:[]})})})]})};t.default=m},3293:function(e,t,n){"use strict";function i(e){return{name:e.name,project_id:e.project_id,schema:JSON.parse(e.schema||'{ "inputs": [], "outputs": [] }')}}n.d(t,{S:function(){return i}})}},function(e){e.O(0,[657,325,590,935,987,425,774,888,179],function(){return e(e.s=9856)}),_N_E=e.O()}]);