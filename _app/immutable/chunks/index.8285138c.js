function h(){}function Y(t,n){for(const e in n)t[e]=n[e];return t}function W(t){return t()}function B(){return Object.create(null)}function p(t){t.forEach(W)}function x(t){return typeof t=="function"}function Z(t,n){return t!=t?n==n:t!==n||t&&typeof t=="object"||typeof t=="function"}function tt(t){return Object.keys(t).length===0}function H(t,...n){if(t==null)return h;const e=t.subscribe(...n);return e.unsubscribe?()=>e.unsubscribe():e}function nt(t){let n;return H(t,e=>n=e)(),n}function et(t,n,e){t.$$.on_destroy.push(H(n,e))}function ot(t,n,e,o){if(t){const r=P(t,n,e,o);return t[0](r)}}function P(t,n,e,o){return t[1]&&o?Y(e.ctx.slice(),t[1](o(n))):e.ctx}function rt(t,n,e,o){if(t[2]&&o){const r=t[2](o(e));if(n.dirty===void 0)return r;if(typeof r=="object"){const a=[],i=Math.max(n.dirty.length,r.length);for(let u=0;u<i;u+=1)a[u]=n.dirty[u]|r[u];return a}return n.dirty|r}return n.dirty}function it(t,n,e,o,r,a){if(r){const i=P(n,e,o,a);t.p(i,r)}}function ct(t){if(t.ctx.length>32){const n=[],e=t.ctx.length/32;for(let o=0;o<e;o++)n[o]=-1;return n}return-1}function st(t){return t??""}function at(t){return t&&x(t.destroy)?t.destroy:h}const ut=["",!0,1,"true","contenteditable"];let v=!1;function lt(){v=!0}function ft(){v=!1}function dt(t,n,e,o){for(;t<n;){const r=t+(n-t>>1);e(r)<=o?t=r+1:n=r}return t}function ht(t){if(t.hydrate_init)return;t.hydrate_init=!0;let n=t.childNodes;if(t.nodeName==="HEAD"){const c=[];for(let s=0;s<n.length;s++){const f=n[s];f.claim_order!==void 0&&c.push(f)}n=c}const e=new Int32Array(n.length+1),o=new Int32Array(n.length);e[0]=-1;let r=0;for(let c=0;c<n.length;c++){const s=n[c].claim_order,f=(r>0&&n[e[r]].claim_order<=s?r+1:dt(1,r,b=>n[e[b]].claim_order,s))-1;o[c]=e[f]+1;const l=f+1;e[l]=c,r=Math.max(l,r)}const a=[],i=[];let u=n.length-1;for(let c=e[r]+1;c!=0;c=o[c-1]){for(a.push(n[c-1]);u>=c;u--)i.push(n[u]);u--}for(;u>=0;u--)i.push(n[u]);a.reverse(),i.sort((c,s)=>c.claim_order-s.claim_order);for(let c=0,s=0;c<i.length;c++){for(;s<a.length&&i[c].claim_order>=a[s].claim_order;)s++;const f=s<a.length?a[s]:null;t.insertBefore(i[c],f)}}function pt(t,n){t.appendChild(n)}function j(t,n){if(v){for(ht(t),(t.actual_end_child===void 0||t.actual_end_child!==null&&t.actual_end_child.parentNode!==t)&&(t.actual_end_child=t.firstChild);t.actual_end_child!==null&&t.actual_end_child.claim_order===void 0;)t.actual_end_child=t.actual_end_child.nextSibling;n!==t.actual_end_child?(n.claim_order!==void 0||n.parentNode!==t)&&t.insertBefore(n,t.actual_end_child):t.actual_end_child=n.nextSibling}else(n.parentNode!==t||n.nextSibling!==null)&&t.appendChild(n)}function _t(t,n,e){v&&!e?j(t,n):(n.parentNode!==t||n.nextSibling!=e)&&t.insertBefore(n,e||null)}function k(t){t.parentNode&&t.parentNode.removeChild(t)}function mt(t,n){for(let e=0;e<t.length;e+=1)t[e]&&t[e].d(n)}function S(t){return document.createElement(t)}function w(t){return document.createTextNode(t)}function $t(){return w(" ")}function gt(){return w("")}function T(t,n,e,o){return t.addEventListener(n,e,o),()=>t.removeEventListener(n,e,o)}function yt(t,n,e){e==null?t.removeAttribute(n):t.getAttribute(n)!==e&&t.setAttribute(n,e)}function bt(t){let n;return{p(...e){n=e,n.forEach(o=>t.push(o))},r(){n.forEach(e=>t.splice(t.indexOf(e),1))}}}function xt(t){return t===""?null:+t}function q(t){return Array.from(t.childNodes)}function vt(t){t.claim_info===void 0&&(t.claim_info={last_index:0,total_claimed:0})}function L(t,n,e,o,r=!1){vt(t);const a=(()=>{for(let i=t.claim_info.last_index;i<t.length;i++){const u=t[i];if(n(u)){const c=e(u);return c===void 0?t.splice(i,1):t[i]=c,r||(t.claim_info.last_index=i),u}}for(let i=t.claim_info.last_index-1;i>=0;i--){const u=t[i];if(n(u)){const c=e(u);return c===void 0?t.splice(i,1):t[i]=c,r?c===void 0&&t.claim_info.last_index--:t.claim_info.last_index=i,u}}return o()})();return a.claim_order=t.claim_info.total_claimed,t.claim_info.total_claimed+=1,a}function wt(t,n,e,o){return L(t,r=>r.nodeName===n,r=>{const a=[];for(let i=0;i<r.attributes.length;i++){const u=r.attributes[i];e[u.name]||a.push(u.name)}a.forEach(i=>r.removeAttribute(i))},()=>o(n))}function Et(t,n,e){return wt(t,n,e,S)}function F(t,n){return L(t,e=>e.nodeType===3,e=>{const o=""+n;if(e.data.startsWith(o)){if(e.data.length!==o.length)return e.splitText(o.length)}else e.data=o},()=>w(n),!0)}function At(t){return F(t," ")}function R(t,n){n=""+n,t.data!==n&&(t.data=n)}function Nt(t,n){n=""+n,t.wholeText!==n&&(t.data=n)}function kt(t,n,e){~ut.indexOf(e)?Nt(t,n):R(t,n)}function St(t,n){t.value=n??""}function Tt(t,n,e,o){e==null?t.style.removeProperty(n):t.style.setProperty(n,e,o?"important":"")}function Ct(t,n,e){for(let o=0;o<t.options.length;o+=1){const r=t.options[o];if(r.__value===n){r.selected=!0;return}}(!e||n!==void 0)&&(t.selectedIndex=-1)}function Ot(t){const n=t.querySelector(":checked");return n&&n.__value}let E;function zt(){if(E===void 0){E=!1;try{typeof window<"u"&&window.parent&&window.parent.document}catch{E=!0}}return E}function Dt(t,n){getComputedStyle(t).position==="static"&&(t.style.position="relative");const e=S("iframe");e.setAttribute("style","display: block; position: absolute; top: 0; left: 0; width: 100%; height: 100%; overflow: hidden; border: 0; opacity: 0; pointer-events: none; z-index: -1;"),e.setAttribute("aria-hidden","true"),e.tabIndex=-1;const o=zt();let r;return o?(e.src="data:text/html,<script>onresize=function(){parent.postMessage(0,'*')}<\/script>",r=T(window,"message",a=>{a.source===e.contentWindow&&n()})):(e.src="about:blank",e.onload=()=>{r=T(e.contentWindow,"resize",n),n()}),pt(t,e),()=>{(o||r&&e.contentWindow)&&r(),k(e)}}function It(t,n){const e=[];let o=0;for(const r of n.childNodes)if(r.nodeType===8){const a=r.textContent.trim();a===`HEAD_${t}_END`?(o-=1,e.push(r)):a===`HEAD_${t}_START`&&(o+=1,e.push(r))}else o>0&&e.push(r);return e}function Mt(t,n){return new t(n)}let g;function y(t){g=t}function G(){if(!g)throw new Error("Function called outside component initialization");return g}function Wt(t){G().$$.on_mount.push(t)}function Bt(t){G().$$.after_update.push(t)}function Ht(t,n){const e=t.$$.callbacks[n.type];e&&e.slice().forEach(o=>o.call(this,n))}const _=[],C=[];let m=[];const O=[],J=Promise.resolve();let z=!1;function K(){z||(z=!0,J.then(Q))}function Pt(){return K(),J}function A(t){m.push(t)}function jt(t){O.push(t)}const D=new Set;let $=0;function Q(){if($!==0)return;const t=g;do{try{for(;$<_.length;){const n=_[$];$++,y(n),qt(n.$$)}}catch(n){throw _.length=0,$=0,n}for(y(null),_.length=0,$=0;C.length;)C.pop()();for(let n=0;n<m.length;n+=1){const e=m[n];D.has(e)||(D.add(e),e())}m.length=0}while(_.length);for(;O.length;)O.pop()();z=!1,D.clear(),y(t)}function qt(t){if(t.fragment!==null){t.update(),p(t.before_update);const n=t.dirty;t.dirty=[-1],t.fragment&&t.fragment.p(t.ctx,n),t.after_update.forEach(A)}}function Lt(t){const n=[],e=[];m.forEach(o=>t.indexOf(o)===-1?n.push(o):e.push(o)),e.forEach(o=>o()),m=n}const N=new Set;let d;function Ft(){d={r:0,c:[],p:d}}function Rt(){d.r||p(d.c),d=d.p}function U(t,n){t&&t.i&&(N.delete(t),t.i(n))}function Gt(t,n,e,o){if(t&&t.o){if(N.has(t))return;N.add(t),d.c.push(()=>{N.delete(t),o&&(e&&t.d(1),o())}),t.o(n)}else o&&o()}function Jt(t,n,e){const o=t.$$.props[n];o!==void 0&&(t.$$.bound[o]=e,e(t.$$.ctx[o]))}function Kt(t){t&&t.c()}function Qt(t,n){t&&t.l(n)}function V(t,n,e,o){const{fragment:r,after_update:a}=t.$$;r&&r.m(n,e),o||A(()=>{const i=t.$$.on_mount.map(W).filter(x);t.$$.on_destroy?t.$$.on_destroy.push(...i):p(i),t.$$.on_mount=[]}),a.forEach(A)}function X(t,n){const e=t.$$;e.fragment!==null&&(Lt(e.after_update),p(e.on_destroy),e.fragment&&e.fragment.d(n),e.on_destroy=e.fragment=null,e.ctx=[])}function Ut(t,n){t.$$.dirty[0]===-1&&(_.push(t),K(),t.$$.dirty.fill(0)),t.$$.dirty[n/31|0]|=1<<n%31}function Vt(t,n,e,o,r,a,i,u=[-1]){const c=g;y(t);const s=t.$$={fragment:null,ctx:[],props:a,update:h,not_equal:r,bound:B(),on_mount:[],on_destroy:[],on_disconnect:[],before_update:[],after_update:[],context:new Map(n.context||(c?c.$$.context:[])),callbacks:B(),dirty:u,skip_bound:!1,root:n.target||c.$$.root};i&&i(s.root);let f=!1;if(s.ctx=e?e(t,n.props||{},(l,b,...I)=>{const M=I.length?I[0]:b;return s.ctx&&r(s.ctx[l],s.ctx[l]=M)&&(!s.skip_bound&&s.bound[l]&&s.bound[l](M),f&&Ut(t,l)),b}):[],s.update(),f=!0,p(s.before_update),s.fragment=o?o(s.ctx):!1,n.target){if(n.hydrate){lt();const l=q(n.target);s.fragment&&s.fragment.l(l),l.forEach(k)}else s.fragment&&s.fragment.c();n.intro&&U(t.$$.fragment),V(t,n.target,n.anchor,n.customElement),ft(),Q()}y(c)}class Xt{$destroy(){X(this,1),this.$destroy=h}$on(n,e){if(!x(e))return h;const o=this.$$.callbacks[n]||(this.$$.callbacks[n]=[]);return o.push(e),()=>{const r=o.indexOf(e);r!==-1&&o.splice(r,1)}}$set(n){this.$$set&&!tt(n)&&(this.$$.skip_bound=!0,this.$$set(n),this.$$.skip_bound=!1)}}export{at as $,V as A,X as B,j as C,h as D,et as E,ot as F,it as G,ct as H,rt as I,mt as J,A as K,Dt as L,T as M,p as N,Ht as O,Ot as P,Ct as Q,xt as R,Xt as S,St as T,kt as U,Jt as V,jt as W,bt as X,It as Y,nt as Z,st as _,$t as a,x as a0,_t as b,At as c,Gt as d,gt as e,Rt as f,U as g,k as h,Vt as i,Bt as j,S as k,Et as l,q as m,yt as n,Wt as o,Tt as p,w as q,F as r,Z as s,Pt as t,R as u,Ft as v,C as w,Mt as x,Kt as y,Qt as z};
