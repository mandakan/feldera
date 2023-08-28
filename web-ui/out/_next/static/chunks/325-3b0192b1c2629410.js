"use strict";(self.webpackChunk_N_E=self.webpackChunk_N_E||[]).push([[325],{6078:function(e,t,n){n.d(t,{Z:function(){return eB}});var o,r,i,a,s,f=n(7462),p=n(3366),c=n(7294),l=n(67),u=n(6600),d=n(7094);function m(e){if(null==e)return window;if("[object Window]"!==e.toString()){var t=e.ownerDocument;return t&&t.defaultView||window}return e}function h(e){var t=m(e).Element;return e instanceof t||e instanceof Element}function v(e){var t=m(e).HTMLElement;return e instanceof t||e instanceof HTMLElement}function y(e){if("undefined"==typeof ShadowRoot)return!1;var t=m(e).ShadowRoot;return e instanceof t||e instanceof ShadowRoot}var g=Math.max,b=Math.min,x=Math.round;function w(){var e=navigator.userAgentData;return null!=e&&e.brands&&Array.isArray(e.brands)?e.brands.map(function(e){return e.brand+"/"+e.version}).join(" "):navigator.userAgent}function O(){return!/^((?!chrome|android).)*safari/i.test(w())}function E(e,t,n){void 0===t&&(t=!1),void 0===n&&(n=!1);var o=e.getBoundingClientRect(),r=1,i=1;t&&v(e)&&(r=e.offsetWidth>0&&x(o.width)/e.offsetWidth||1,i=e.offsetHeight>0&&x(o.height)/e.offsetHeight||1);var a=(h(e)?m(e):window).visualViewport,s=!O()&&n,f=(o.left+(s&&a?a.offsetLeft:0))/r,p=(o.top+(s&&a?a.offsetTop:0))/i,c=o.width/r,l=o.height/i;return{width:c,height:l,top:p,right:f+c,bottom:p+l,left:f,x:f,y:p}}function P(e){var t=m(e);return{scrollLeft:t.pageXOffset,scrollTop:t.pageYOffset}}function j(e){return e?(e.nodeName||"").toLowerCase():null}function D(e){return((h(e)?e.ownerDocument:e.document)||window.document).documentElement}function R(e){return E(D(e)).left+P(e).scrollLeft}function A(e){return m(e).getComputedStyle(e)}function k(e){var t=A(e),n=t.overflow,o=t.overflowX,r=t.overflowY;return/auto|scroll|overlay|hidden/.test(n+r+o)}function M(e){var t=E(e),n=e.offsetWidth,o=e.offsetHeight;return 1>=Math.abs(t.width-n)&&(n=t.width),1>=Math.abs(t.height-o)&&(o=t.height),{x:e.offsetLeft,y:e.offsetTop,width:n,height:o}}function Z(e){return"html"===j(e)?e:e.assignedSlot||e.parentNode||(y(e)?e.host:null)||D(e)}function L(e,t){void 0===t&&(t=[]);var n,o=function e(t){return["html","body","#document"].indexOf(j(t))>=0?t.ownerDocument.body:v(t)&&k(t)?t:e(Z(t))}(e),r=o===(null==(n=e.ownerDocument)?void 0:n.body),i=m(o),a=r?[i].concat(i.visualViewport||[],k(o)?o:[]):o,s=t.concat(a);return r?s:s.concat(L(Z(a)))}function T(e){return v(e)&&"fixed"!==A(e).position?e.offsetParent:null}function W(e){for(var t=m(e),n=T(e);n&&["table","td","th"].indexOf(j(n))>=0&&"static"===A(n).position;)n=T(n);return n&&("html"===j(n)||"body"===j(n)&&"static"===A(n).position)?t:n||function(e){var t=/firefox/i.test(w());if(/Trident/i.test(w())&&v(e)&&"fixed"===A(e).position)return null;var n=Z(e);for(y(n)&&(n=n.host);v(n)&&0>["html","body"].indexOf(j(n));){var o=A(n);if("none"!==o.transform||"none"!==o.perspective||"paint"===o.contain||-1!==["transform","perspective"].indexOf(o.willChange)||t&&"filter"===o.willChange||t&&o.filter&&"none"!==o.filter)return n;n=n.parentNode}return null}(e)||t}var S="bottom",B="right",H="left",C="auto",V=["top",S,B,H],q="start",N="viewport",_="popper",I=V.reduce(function(e,t){return e.concat([t+"-"+q,t+"-end"])},[]),U=[].concat(V,[C]).reduce(function(e,t){return e.concat([t,t+"-"+q,t+"-end"])},[]),F=["beforeRead","read","afterRead","beforeMain","main","afterMain","beforeWrite","write","afterWrite"],z={placement:"bottom",modifiers:[],strategy:"absolute"};function X(){for(var e=arguments.length,t=Array(e),n=0;n<e;n++)t[n]=arguments[n];return!t.some(function(e){return!(e&&"function"==typeof e.getBoundingClientRect)})}var Y={passive:!0};function $(e){return e.split("-")[0]}function G(e){return e.split("-")[1]}function J(e){return["top","bottom"].indexOf(e)>=0?"x":"y"}function K(e){var t,n=e.reference,o=e.element,r=e.placement,i=r?$(r):null,a=r?G(r):null,s=n.x+n.width/2-o.width/2,f=n.y+n.height/2-o.height/2;switch(i){case"top":t={x:s,y:n.y-o.height};break;case S:t={x:s,y:n.y+n.height};break;case B:t={x:n.x+n.width,y:f};break;case H:t={x:n.x-o.width,y:f};break;default:t={x:n.x,y:n.y}}var p=i?J(i):null;if(null!=p){var c="y"===p?"height":"width";switch(a){case q:t[p]=t[p]-(n[c]/2-o[c]/2);break;case"end":t[p]=t[p]+(n[c]/2-o[c]/2)}}return t}var Q={top:"auto",right:"auto",bottom:"auto",left:"auto"};function ee(e){var t,n,o,r,i,a,s,f=e.popper,p=e.popperRect,c=e.placement,l=e.variation,u=e.offsets,d=e.position,h=e.gpuAcceleration,v=e.adaptive,y=e.roundOffsets,g=e.isFixed,b=u.x,w=void 0===b?0:b,O=u.y,E=void 0===O?0:O,P="function"==typeof y?y({x:w,y:E}):{x:w,y:E};w=P.x,E=P.y;var j=u.hasOwnProperty("x"),R=u.hasOwnProperty("y"),k=H,M="top",Z=window;if(v){var L=W(f),T="clientHeight",C="clientWidth";L===m(f)&&"static"!==A(L=D(f)).position&&"absolute"===d&&(T="scrollHeight",C="scrollWidth"),("top"===c||(c===H||c===B)&&"end"===l)&&(M=S,E-=(g&&L===Z&&Z.visualViewport?Z.visualViewport.height:L[T])-p.height,E*=h?1:-1),(c===H||("top"===c||c===S)&&"end"===l)&&(k=B,w-=(g&&L===Z&&Z.visualViewport?Z.visualViewport.width:L[C])-p.width,w*=h?1:-1)}var V=Object.assign({position:d},v&&Q),q=!0===y?(t={x:w,y:E},n=m(f),o=t.x,r=t.y,{x:x(o*(i=n.devicePixelRatio||1))/i||0,y:x(r*i)/i||0}):{x:w,y:E};return(w=q.x,E=q.y,h)?Object.assign({},V,((s={})[M]=R?"0":"",s[k]=j?"0":"",s.transform=1>=(Z.devicePixelRatio||1)?"translate("+w+"px, "+E+"px)":"translate3d("+w+"px, "+E+"px, 0)",s)):Object.assign({},V,((a={})[M]=R?E+"px":"",a[k]=j?w+"px":"",a.transform="",a))}var et={left:"right",right:"left",bottom:"top",top:"bottom"};function en(e){return e.replace(/left|right|bottom|top/g,function(e){return et[e]})}var eo={start:"end",end:"start"};function er(e){return e.replace(/start|end/g,function(e){return eo[e]})}function ei(e,t){var n=t.getRootNode&&t.getRootNode();if(e.contains(t))return!0;if(n&&y(n)){var o=t;do{if(o&&e.isSameNode(o))return!0;o=o.parentNode||o.host}while(o)}return!1}function ea(e){return Object.assign({},e,{left:e.x,top:e.y,right:e.x+e.width,bottom:e.y+e.height})}function es(e,t,n){var o,r,i,a,s,f,p,c,l,u;return t===N?ea(function(e,t){var n=m(e),o=D(e),r=n.visualViewport,i=o.clientWidth,a=o.clientHeight,s=0,f=0;if(r){i=r.width,a=r.height;var p=O();(p||!p&&"fixed"===t)&&(s=r.offsetLeft,f=r.offsetTop)}return{width:i,height:a,x:s+R(e),y:f}}(e,n)):h(t)?((o=E(t,!1,"fixed"===n)).top=o.top+t.clientTop,o.left=o.left+t.clientLeft,o.bottom=o.top+t.clientHeight,o.right=o.left+t.clientWidth,o.width=t.clientWidth,o.height=t.clientHeight,o.x=o.left,o.y=o.top,o):ea((r=D(e),a=D(r),s=P(r),f=null==(i=r.ownerDocument)?void 0:i.body,p=g(a.scrollWidth,a.clientWidth,f?f.scrollWidth:0,f?f.clientWidth:0),c=g(a.scrollHeight,a.clientHeight,f?f.scrollHeight:0,f?f.clientHeight:0),l=-s.scrollLeft+R(r),u=-s.scrollTop,"rtl"===A(f||a).direction&&(l+=g(a.clientWidth,f?f.clientWidth:0)-p),{width:p,height:c,x:l,y:u}))}function ef(){return{top:0,right:0,bottom:0,left:0}}function ep(e){return Object.assign({},ef(),e)}function ec(e,t){return t.reduce(function(t,n){return t[n]=e,t},{})}function el(e,t){void 0===t&&(t={});var n,o,r,i,a,s,f,p=t,c=p.placement,l=void 0===c?e.placement:c,u=p.strategy,d=void 0===u?e.strategy:u,m=p.boundary,y=p.rootBoundary,x=p.elementContext,w=void 0===x?_:x,O=p.altBoundary,P=p.padding,R=void 0===P?0:P,k=ep("number"!=typeof R?R:ec(R,V)),M=e.rects.popper,T=e.elements[void 0!==O&&O?w===_?"reference":_:w],H=(n=h(T)?T:T.contextElement||D(e.elements.popper),s=(a=[].concat("clippingParents"===(o=void 0===m?"clippingParents":m)?(r=L(Z(n)),h(i=["absolute","fixed"].indexOf(A(n).position)>=0&&v(n)?W(n):n)?r.filter(function(e){return h(e)&&ei(e,i)&&"body"!==j(e)}):[]):[].concat(o),[void 0===y?N:y]))[0],(f=a.reduce(function(e,t){var o=es(n,t,d);return e.top=g(o.top,e.top),e.right=b(o.right,e.right),e.bottom=b(o.bottom,e.bottom),e.left=g(o.left,e.left),e},es(n,s,d))).width=f.right-f.left,f.height=f.bottom-f.top,f.x=f.left,f.y=f.top,f),C=E(e.elements.reference),q=K({reference:C,element:M,strategy:"absolute",placement:l}),I=ea(Object.assign({},M,q)),U=w===_?I:C,F={top:H.top-U.top+k.top,bottom:U.bottom-H.bottom+k.bottom,left:H.left-U.left+k.left,right:U.right-H.right+k.right},z=e.modifiersData.offset;if(w===_&&z){var X=z[l];Object.keys(F).forEach(function(e){var t=[B,S].indexOf(e)>=0?1:-1,n=["top",S].indexOf(e)>=0?"y":"x";F[e]+=X[n]*t})}return F}function eu(e,t,n){return g(e,b(t,n))}function ed(e,t,n){return void 0===n&&(n={x:0,y:0}),{top:e.top-t.height-n.y,right:e.right-t.width+n.x,bottom:e.bottom-t.height+n.y,left:e.left-t.width-n.x}}function em(e){return["top",B,S,H].some(function(t){return e[t]>=0})}var eh=(i=void 0===(r=(o={defaultModifiers:[{name:"eventListeners",enabled:!0,phase:"write",fn:function(){},effect:function(e){var t=e.state,n=e.instance,o=e.options,r=o.scroll,i=void 0===r||r,a=o.resize,s=void 0===a||a,f=m(t.elements.popper),p=[].concat(t.scrollParents.reference,t.scrollParents.popper);return i&&p.forEach(function(e){e.addEventListener("scroll",n.update,Y)}),s&&f.addEventListener("resize",n.update,Y),function(){i&&p.forEach(function(e){e.removeEventListener("scroll",n.update,Y)}),s&&f.removeEventListener("resize",n.update,Y)}},data:{}},{name:"popperOffsets",enabled:!0,phase:"read",fn:function(e){var t=e.state,n=e.name;t.modifiersData[n]=K({reference:t.rects.reference,element:t.rects.popper,strategy:"absolute",placement:t.placement})},data:{}},{name:"computeStyles",enabled:!0,phase:"beforeWrite",fn:function(e){var t=e.state,n=e.options,o=n.gpuAcceleration,r=n.adaptive,i=n.roundOffsets,a=void 0===i||i,s={placement:$(t.placement),variation:G(t.placement),popper:t.elements.popper,popperRect:t.rects.popper,gpuAcceleration:void 0===o||o,isFixed:"fixed"===t.options.strategy};null!=t.modifiersData.popperOffsets&&(t.styles.popper=Object.assign({},t.styles.popper,ee(Object.assign({},s,{offsets:t.modifiersData.popperOffsets,position:t.options.strategy,adaptive:void 0===r||r,roundOffsets:a})))),null!=t.modifiersData.arrow&&(t.styles.arrow=Object.assign({},t.styles.arrow,ee(Object.assign({},s,{offsets:t.modifiersData.arrow,position:"absolute",adaptive:!1,roundOffsets:a})))),t.attributes.popper=Object.assign({},t.attributes.popper,{"data-popper-placement":t.placement})},data:{}},{name:"applyStyles",enabled:!0,phase:"write",fn:function(e){var t=e.state;Object.keys(t.elements).forEach(function(e){var n=t.styles[e]||{},o=t.attributes[e]||{},r=t.elements[e];v(r)&&j(r)&&(Object.assign(r.style,n),Object.keys(o).forEach(function(e){var t=o[e];!1===t?r.removeAttribute(e):r.setAttribute(e,!0===t?"":t)}))})},effect:function(e){var t=e.state,n={popper:{position:t.options.strategy,left:"0",top:"0",margin:"0"},arrow:{position:"absolute"},reference:{}};return Object.assign(t.elements.popper.style,n.popper),t.styles=n,t.elements.arrow&&Object.assign(t.elements.arrow.style,n.arrow),function(){Object.keys(t.elements).forEach(function(e){var o=t.elements[e],r=t.attributes[e]||{},i=Object.keys(t.styles.hasOwnProperty(e)?t.styles[e]:n[e]).reduce(function(e,t){return e[t]="",e},{});v(o)&&j(o)&&(Object.assign(o.style,i),Object.keys(r).forEach(function(e){o.removeAttribute(e)}))})}},requires:["computeStyles"]},{name:"offset",enabled:!0,phase:"main",requires:["popperOffsets"],fn:function(e){var t=e.state,n=e.options,o=e.name,r=n.offset,i=void 0===r?[0,0]:r,a=U.reduce(function(e,n){var o,r,a,s,f,p;return e[n]=(o=t.rects,a=[H,"top"].indexOf(r=$(n))>=0?-1:1,f=(s="function"==typeof i?i(Object.assign({},o,{placement:n})):i)[0],p=s[1],f=f||0,p=(p||0)*a,[H,B].indexOf(r)>=0?{x:p,y:f}:{x:f,y:p}),e},{}),s=a[t.placement],f=s.x,p=s.y;null!=t.modifiersData.popperOffsets&&(t.modifiersData.popperOffsets.x+=f,t.modifiersData.popperOffsets.y+=p),t.modifiersData[o]=a}},{name:"flip",enabled:!0,phase:"main",fn:function(e){var t=e.state,n=e.options,o=e.name;if(!t.modifiersData[o]._skip){for(var r=n.mainAxis,i=void 0===r||r,a=n.altAxis,s=void 0===a||a,f=n.fallbackPlacements,p=n.padding,c=n.boundary,l=n.rootBoundary,u=n.altBoundary,d=n.flipVariations,m=void 0===d||d,h=n.allowedAutoPlacements,v=t.options.placement,y=$(v),g=[v].concat(f||(y!==v&&m?function(e){if($(e)===C)return[];var t=en(e);return[er(e),t,er(t)]}(v):[en(v)])).reduce(function(e,n){var o,r,i,a,s,f,u,d,v,y,g,b;return e.concat($(n)===C?(r=(o={placement:n,boundary:c,rootBoundary:l,padding:p,flipVariations:m,allowedAutoPlacements:h}).placement,i=o.boundary,a=o.rootBoundary,s=o.padding,f=o.flipVariations,d=void 0===(u=o.allowedAutoPlacements)?U:u,0===(g=(y=(v=G(r))?f?I:I.filter(function(e){return G(e)===v}):V).filter(function(e){return d.indexOf(e)>=0})).length&&(g=y),Object.keys(b=g.reduce(function(e,n){return e[n]=el(t,{placement:n,boundary:i,rootBoundary:a,padding:s})[$(n)],e},{})).sort(function(e,t){return b[e]-b[t]})):n)},[]),b=t.rects.reference,x=t.rects.popper,w=new Map,O=!0,E=g[0],P=0;P<g.length;P++){var j=g[P],D=$(j),R=G(j)===q,A=["top",S].indexOf(D)>=0,k=A?"width":"height",M=el(t,{placement:j,boundary:c,rootBoundary:l,altBoundary:u,padding:p}),Z=A?R?B:H:R?S:"top";b[k]>x[k]&&(Z=en(Z));var L=en(Z),T=[];if(i&&T.push(M[D]<=0),s&&T.push(M[Z]<=0,M[L]<=0),T.every(function(e){return e})){E=j,O=!1;break}w.set(j,T)}if(O)for(var W=m?3:1,N=function(e){var t=g.find(function(t){var n=w.get(t);if(n)return n.slice(0,e).every(function(e){return e})});if(t)return E=t,"break"},_=W;_>0&&"break"!==N(_);_--);t.placement!==E&&(t.modifiersData[o]._skip=!0,t.placement=E,t.reset=!0)}},requiresIfExists:["offset"],data:{_skip:!1}},{name:"preventOverflow",enabled:!0,phase:"main",fn:function(e){var t=e.state,n=e.options,o=e.name,r=n.mainAxis,i=n.altAxis,a=n.boundary,s=n.rootBoundary,f=n.altBoundary,p=n.padding,c=n.tether,l=void 0===c||c,u=n.tetherOffset,d=void 0===u?0:u,m=el(t,{boundary:a,rootBoundary:s,padding:p,altBoundary:f}),h=$(t.placement),v=G(t.placement),y=!v,x=J(h),w="x"===x?"y":"x",O=t.modifiersData.popperOffsets,E=t.rects.reference,P=t.rects.popper,j="function"==typeof d?d(Object.assign({},t.rects,{placement:t.placement})):d,D="number"==typeof j?{mainAxis:j,altAxis:j}:Object.assign({mainAxis:0,altAxis:0},j),R=t.modifiersData.offset?t.modifiersData.offset[t.placement]:null,A={x:0,y:0};if(O){if(void 0===r||r){var k,Z="y"===x?"top":H,L="y"===x?S:B,T="y"===x?"height":"width",C=O[x],V=C+m[Z],N=C-m[L],_=l?-P[T]/2:0,I=v===q?E[T]:P[T],U=v===q?-P[T]:-E[T],F=t.elements.arrow,z=l&&F?M(F):{width:0,height:0},X=t.modifiersData["arrow#persistent"]?t.modifiersData["arrow#persistent"].padding:ef(),Y=X[Z],K=X[L],Q=eu(0,E[T],z[T]),ee=y?E[T]/2-_-Q-Y-D.mainAxis:I-Q-Y-D.mainAxis,et=y?-E[T]/2+_+Q+K+D.mainAxis:U+Q+K+D.mainAxis,en=t.elements.arrow&&W(t.elements.arrow),eo=en?"y"===x?en.clientTop||0:en.clientLeft||0:0,er=null!=(k=null==R?void 0:R[x])?k:0,ei=eu(l?b(V,C+ee-er-eo):V,C,l?g(N,C+et-er):N);O[x]=ei,A[x]=ei-C}if(void 0!==i&&i){var ea,es,ep=O[w],ec="y"===w?"height":"width",ed=ep+m["x"===x?"top":H],em=ep-m["x"===x?S:B],eh=-1!==["top",H].indexOf(h),ev=null!=(ea=null==R?void 0:R[w])?ea:0,ey=eh?ed:ep-E[ec]-P[ec]-ev+D.altAxis,eg=eh?ep+E[ec]+P[ec]-ev-D.altAxis:em,eb=l&&eh?(es=eu(ey,ep,eg))>eg?eg:es:eu(l?ey:ed,ep,l?eg:em);O[w]=eb,A[w]=eb-ep}t.modifiersData[o]=A}},requiresIfExists:["offset"]},{name:"arrow",enabled:!0,phase:"main",fn:function(e){var t,n,o=e.state,r=e.name,i=e.options,a=o.elements.arrow,s=o.modifiersData.popperOffsets,f=$(o.placement),p=J(f),c=[H,B].indexOf(f)>=0?"height":"width";if(a&&s){var l=ep("number"!=typeof(t="function"==typeof(t=i.padding)?t(Object.assign({},o.rects,{placement:o.placement})):t)?t:ec(t,V)),u=M(a),d=o.rects.reference[c]+o.rects.reference[p]-s[p]-o.rects.popper[c],m=s[p]-o.rects.reference[p],h=W(a),v=h?"y"===p?h.clientHeight||0:h.clientWidth||0:0,y=l["y"===p?"top":H],g=v-u[c]-l["y"===p?S:B],b=v/2-u[c]/2+(d/2-m/2),x=eu(y,b,g);o.modifiersData[r]=((n={})[p]=x,n.centerOffset=x-b,n)}},effect:function(e){var t=e.state,n=e.options.element,o=void 0===n?"[data-popper-arrow]":n;null!=o&&("string"!=typeof o||(o=t.elements.popper.querySelector(o)))&&ei(t.elements.popper,o)&&(t.elements.arrow=o)},requires:["popperOffsets"],requiresIfExists:["preventOverflow"]},{name:"hide",enabled:!0,phase:"main",requiresIfExists:["preventOverflow"],fn:function(e){var t=e.state,n=e.name,o=t.rects.reference,r=t.rects.popper,i=t.modifiersData.preventOverflow,a=el(t,{elementContext:"reference"}),s=el(t,{altBoundary:!0}),f=ed(a,o),p=ed(s,r,i),c=em(f),l=em(p);t.modifiersData[n]={referenceClippingOffsets:f,popperEscapeOffsets:p,isReferenceHidden:c,hasPopperEscaped:l},t.attributes.popper=Object.assign({},t.attributes.popper,{"data-popper-reference-hidden":c,"data-popper-escaped":l})}}]}).defaultModifiers)?[]:r,s=void 0===(a=o.defaultOptions)?z:a,function(e,t,n){void 0===n&&(n=s);var o,r={placement:"bottom",orderedModifiers:[],options:Object.assign({},z,s),modifiersData:{},elements:{reference:e,popper:t},attributes:{},styles:{}},a=[],f=!1,p={state:r,setOptions:function(n){var o,f,l,u,d,m="function"==typeof n?n(r.options):n;c(),r.options=Object.assign({},s,r.options,m),r.scrollParents={reference:h(e)?L(e):e.contextElement?L(e.contextElement):[],popper:L(t)};var v=(f=Object.keys(o=[].concat(i,r.options.modifiers).reduce(function(e,t){var n=e[t.name];return e[t.name]=n?Object.assign({},n,t,{options:Object.assign({},n.options,t.options),data:Object.assign({},n.data,t.data)}):t,e},{})).map(function(e){return o[e]}),l=new Map,u=new Set,d=[],f.forEach(function(e){l.set(e.name,e)}),f.forEach(function(e){u.has(e.name)||function e(t){u.add(t.name),[].concat(t.requires||[],t.requiresIfExists||[]).forEach(function(t){if(!u.has(t)){var n=l.get(t);n&&e(n)}}),d.push(t)}(e)}),F.reduce(function(e,t){return e.concat(d.filter(function(e){return e.phase===t}))},[]));return r.orderedModifiers=v.filter(function(e){return e.enabled}),r.orderedModifiers.forEach(function(e){var t=e.name,n=e.options,o=e.effect;if("function"==typeof o){var i=o({state:r,name:t,instance:p,options:void 0===n?{}:n});a.push(i||function(){})}}),p.update()},forceUpdate:function(){if(!f){var e,t,n,o,i,a,s,c,l,u,d,h,y=r.elements,g=y.reference,b=y.popper;if(X(g,b)){r.rects={reference:(t=W(b),n="fixed"===r.options.strategy,o=v(t),c=v(t)&&(a=x((i=t.getBoundingClientRect()).width)/t.offsetWidth||1,s=x(i.height)/t.offsetHeight||1,1!==a||1!==s),l=D(t),u=E(g,c,n),d={scrollLeft:0,scrollTop:0},h={x:0,y:0},(o||!o&&!n)&&(("body"!==j(t)||k(l))&&(d=(e=t)!==m(e)&&v(e)?{scrollLeft:e.scrollLeft,scrollTop:e.scrollTop}:P(e)),v(t)?(h=E(t,!0),h.x+=t.clientLeft,h.y+=t.clientTop):l&&(h.x=R(l))),{x:u.left+d.scrollLeft-h.x,y:u.top+d.scrollTop-h.y,width:u.width,height:u.height}),popper:M(b)},r.reset=!1,r.placement=r.options.placement,r.orderedModifiers.forEach(function(e){return r.modifiersData[e.name]=Object.assign({},e.data)});for(var w=0;w<r.orderedModifiers.length;w++){if(!0===r.reset){r.reset=!1,w=-1;continue}var O=r.orderedModifiers[w],A=O.fn,Z=O.options,L=void 0===Z?{}:Z,T=O.name;"function"==typeof A&&(r=A({state:r,options:L,name:T,instance:p})||r)}}}},update:function(){return o||(o=new Promise(function(e){Promise.resolve().then(function(){o=void 0,e(new Promise(function(e){p.forceUpdate(),e(r)}))})})),o},destroy:function(){c(),f=!0}};if(!X(e,t))return p;function c(){a.forEach(function(e){return e()}),a=[]}return p.setOptions(n).then(function(e){!f&&n.onFirstUpdate&&n.onFirstUpdate(e)}),p}),ev=n(4780),ey=n(8385),eg=n(4867);function eb(e){return(0,eg.Z)("MuiPopper",e)}(0,n(1588).Z)("MuiPopper",["root"]);var ex=n(4349),ew=n(1873),eO=n(5893);let eE=["anchorEl","children","component","direction","disablePortal","modifiers","open","placement","popperOptions","popperRef","slotProps","slots","TransitionProps","ownerState"],eP=["anchorEl","children","container","direction","disablePortal","keepMounted","modifiers","open","placement","popperOptions","popperRef","style","transition","slotProps","slots"];function ej(e){return"function"==typeof e?e():e}let eD=()=>(0,ev.Z)({root:["root"]},(0,ew.T)(eb)),eR={},eA=c.forwardRef(function(e,t){var n;let{anchorEl:o,children:r,component:i,direction:a,disablePortal:s,modifiers:d,open:m,placement:h,popperOptions:v,popperRef:y,slotProps:g={},slots:b={},TransitionProps:x}=e,w=(0,p.Z)(e,eE),O=c.useRef(null),E=(0,l.Z)(O,t),P=c.useRef(null),j=(0,l.Z)(P,y),D=c.useRef(j);(0,u.Z)(()=>{D.current=j},[j]),c.useImperativeHandle(y,()=>P.current,[]);let R=function(e,t){if("ltr"===t)return e;switch(e){case"bottom-end":return"bottom-start";case"bottom-start":return"bottom-end";case"top-end":return"top-start";case"top-start":return"top-end";default:return e}}(h,a),[A,k]=c.useState(R),[M,Z]=c.useState(ej(o));c.useEffect(()=>{P.current&&P.current.forceUpdate()}),c.useEffect(()=>{o&&Z(ej(o))},[o]),(0,u.Z)(()=>{if(!M||!m)return;let e=e=>{k(e.placement)},t=[{name:"preventOverflow",options:{altBoundary:s}},{name:"flip",options:{altBoundary:s}},{name:"onUpdate",enabled:!0,phase:"afterWrite",fn:({state:t})=>{e(t)}}];null!=d&&(t=t.concat(d)),v&&null!=v.modifiers&&(t=t.concat(v.modifiers));let n=eh(M,O.current,(0,f.Z)({placement:R},v,{modifiers:t}));return D.current(n),()=>{n.destroy(),D.current(null)}},[M,s,d,m,v,R]);let L={placement:A};null!==x&&(L.TransitionProps=x);let T=eD(),W=null!=(n=null!=i?i:b.root)?n:"div",S=(0,ex.Z)({elementType:W,externalSlotProps:g.root,externalForwardedProps:w,additionalProps:{role:"tooltip",ref:E},ownerState:e,className:T.root});return(0,eO.jsx)(W,(0,f.Z)({},S,{children:"function"==typeof r?r(L):r}))}),ek=c.forwardRef(function(e,t){let n;let{anchorEl:o,children:r,container:i,direction:a="ltr",disablePortal:s=!1,keepMounted:l=!1,modifiers:u,open:m,placement:h="bottom",popperOptions:v=eR,popperRef:y,style:g,transition:b=!1,slotProps:x={},slots:w={}}=e,O=(0,p.Z)(e,eP),[E,P]=c.useState(!0),j=()=>{P(!1)},D=()=>{P(!0)};if(!l&&!m&&(!b||E))return null;if(i)n=i;else if(o){let e=ej(o);n=e&&void 0!==e.nodeType?(0,d.Z)(e).body:(0,d.Z)(null).body}return(0,eO.jsx)(ey.Z,{disablePortal:s,container:n,children:(0,eO.jsx)(eA,(0,f.Z)({anchorEl:o,direction:a,disablePortal:s,modifiers:u,ref:t,open:b?!E:m,placement:h,popperOptions:v,popperRef:y,slotProps:x,slots:w},O,{style:(0,f.Z)({position:"fixed",top:0,left:0,display:!m&&l&&(!b||E)?"none":void 0},g),TransitionProps:b?{in:m,onEnter:j,onExited:D}:void 0,children:r}))})});var eM=n(4168),eZ=n(948),eL=n(1657);let eT=["anchorEl","component","components","componentsProps","container","disablePortal","keepMounted","modifiers","open","placement","popperOptions","popperRef","transition","slots","slotProps"],eW=(0,eZ.ZP)(ek,{name:"MuiPopper",slot:"Root",overridesResolver:(e,t)=>t.root})({}),eS=c.forwardRef(function(e,t){var n;let o=(0,eM.Z)(),r=(0,eL.Z)({props:e,name:"MuiPopper"}),{anchorEl:i,component:a,components:s,componentsProps:c,container:l,disablePortal:u,keepMounted:d,modifiers:m,open:h,placement:v,popperOptions:y,popperRef:g,transition:b,slots:x,slotProps:w}=r,O=(0,p.Z)(r,eT),E=null!=(n=null==x?void 0:x.root)?n:null==s?void 0:s.Root,P=(0,f.Z)({anchorEl:i,component:a,container:l,disablePortal:u,keepMounted:d,modifiers:m,open:h,placement:v,popperOptions:y,popperRef:g,transition:b},O);return(0,eO.jsx)(eW,(0,f.Z)({direction:null==o?void 0:o.direction,slots:{root:E},slotProps:null!=w?w:c},P,{ref:t}))});var eB=eS},464:function(e,t,n){n.d(t,{$:function(){return r}});var o=n(1688);let r=o.useSyncExternalStore},4798:function(e,t,n){n.d(t,{L:function(){return o}});function o(e,t){return"function"==typeof e?e(...t):!!e}}}]);