var r,u;import{D as f,s as g}from"./index.8285138c.js";const e=[];function p(n,b=f){let o;const l=new Set;function a(t){if(g(n,t)&&(n=t,o)){const c=!e.length;for(const s of l)s[1](),e.push(s,n);if(c){for(let s=0;s<e.length;s+=2)e[s][0](e[s+1]);e.length=0}}}function h(t){a(t(n))}function _(t,c=f){const s=[t,c];return l.add(s),l.size===1&&(o=b(a)||f),t(n),()=>{l.delete(s),l.size===0&&o&&(o(),o=null)}}return{set:a,update:h,subscribe:_}}const i=((r=globalThis.__sveltekit_szwmae)==null?void 0:r.base)??"/pixel",d=((u=globalThis.__sveltekit_szwmae)==null?void 0:u.assets)??i;export{d as a,i as b,p as w};
