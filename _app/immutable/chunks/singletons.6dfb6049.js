var v,_;import{w as i}from"./index.4b5f902b.js";const p=((v=globalThis.__sveltekit_1b8le1e)==null?void 0:v.base)??"/pixel",w=((_=globalThis.__sveltekit_1b8le1e)==null?void 0:_.assets)??p,y="1702944009641",A="sveltekit:snapshot",x="sveltekit:scroll",T="sveltekit:index",f={tap:1,hover:2,viewport:3,eager:4,off:-1,false:-1},d=location.origin;function U(t){let e=t.baseURI;if(!e){const n=t.getElementsByTagName("base");e=n.length?n[0].href:t.URL}return e}function R(){return{x:pageXOffset,y:pageYOffset}}function u(t,e){return t.getAttribute(`data-sveltekit-${e}`)}const h={...f,"":f.hover};function b(t){let e=t.assignedSlot??t.parentNode;return(e==null?void 0:e.nodeType)===11&&(e=e.host),e}function S(t,e){for(;t&&t!==e;){if(t.nodeName.toUpperCase()==="A"&&t.hasAttribute("href"))return t;t=b(t)}}function E(t,e){let n;try{n=new URL(t instanceof SVGAElement?t.href.baseVal:t.href,document.baseURI)}catch{}const r=t instanceof SVGAElement?t.target.baseVal:t.target,o=!n||!!r||m(n,e)||(t.getAttribute("rel")||"").split(/\s+/).includes("external"),s=(n==null?void 0:n.origin)===d&&t.hasAttribute("download");return{url:n,external:o,target:r,download:s}}function I(t){let e=null,n=null,r=null,o=null,s=null,l=null,a=t;for(;a&&a!==document.documentElement;)r===null&&(r=u(a,"preload-code")),o===null&&(o=u(a,"preload-data")),e===null&&(e=u(a,"keepfocus")),n===null&&(n=u(a,"noscroll")),s===null&&(s=u(a,"reload")),l===null&&(l=u(a,"replacestate")),a=b(a);function c(k){switch(k){case"":case"true":return!0;case"off":case"false":return!1;default:return null}}return{preload_code:h[r??"off"],preload_data:h[o??"off"],keep_focus:c(e),noscroll:c(n),reload:c(s),replace_state:c(l)}}function g(t){const e=i(t);let n=!0;function r(){n=!0,e.update(l=>l)}function o(l){n=!1,e.set(l)}function s(l){let a;return e.subscribe(c=>{(a===void 0||n&&c!==a)&&l(a=c)})}return{notify:r,set:o,subscribe:s}}function V(){const{set:t,subscribe:e}=i(!1);let n;async function r(){clearTimeout(n);try{const o=await fetch(`${w}/_app/version.json`,{headers:{pragma:"no-cache","cache-control":"no-cache"}});if(!o.ok)return!1;const s=(await o.json()).version!==y;return s&&(t(!0),clearTimeout(n)),s}catch{return!1}}return{subscribe:e,check:r}}function m(t,e){return t.origin!==d||!t.pathname.startsWith(e)}function L(t){t.client}const N={url:g({}),page:g({}),navigating:i(null),updated:V()};export{T as I,f as P,x as S,A as a,E as b,I as c,N as d,p as e,S as f,U as g,L as h,m as i,d as o,R as s};
