(function(){const n=document.createElement("link").relList;if(n&&n.supports&&n.supports("modulepreload"))return;for(const o of document.querySelectorAll('link[rel="modulepreload"]'))s(o);new MutationObserver(o=>{for(const i of o)if(i.type==="childList")for(const r of i.addedNodes)r.tagName==="LINK"&&r.rel==="modulepreload"&&s(r)}).observe(document,{childList:!0,subtree:!0});function t(o){const i={};return o.integrity&&(i.integrity=o.integrity),o.referrerpolicy&&(i.referrerPolicy=o.referrerpolicy),o.crossorigin==="use-credentials"?i.credentials="include":o.crossorigin==="anonymous"?i.credentials="omit":i.credentials="same-origin",i}function s(o){if(o.ep)return;o.ep=!0;const i=t(o);fetch(o.href,i)}})();const y={},pe=(e,n)=>e===n,U={equals:pe};let ge=re;const S=1,q=2,ne={owned:null,cleanups:null,context:null,owner:null};var m=null;let w=null,a=null,h=null,v=null,H=0;function me(e,n){const t=a,s=m,o=e.length===0,i=o?ne:{owned:null,cleanups:null,context:null,owner:n||s},r=o?e:()=>e(()=>J(()=>Q(i)));m=i,a=null;try{return I(r,!0)}finally{a=t,m=s}}function $e(e,n){n=n?Object.assign({},U,n):U;const t={value:e,observers:null,observerSlots:null,comparator:n.equals||void 0},s=o=>(typeof o=="function"&&(o=o(t.value)),oe(t,o));return[se.bind(t),s]}function E(e,n,t){const s=le(e,n,!1,S);F(s)}function L(e,n,t){t=t?Object.assign({},U,t):U;const s=le(e,n,!0,0);return s.observers=null,s.observerSlots=null,s.comparator=t.equals||void 0,F(s),se.bind(s)}function J(e){const n=a;a=null;try{return e()}finally{a=n}}function se(){const e=w;if(this.sources&&(this.state||e))if(this.state===S||e)F(this);else{const n=h;h=null,I(()=>j(this),!1),h=n}if(a){const n=this.observers?this.observers.length:0;a.sources?(a.sources.push(this),a.sourceSlots.push(n)):(a.sources=[this],a.sourceSlots=[n]),this.observers?(this.observers.push(a),this.observerSlots.push(a.sources.length-1)):(this.observers=[a],this.observerSlots=[a.sources.length-1])}return this.value}function oe(e,n,t){let s=e.value;return(!e.comparator||!e.comparator(s,n))&&(e.value=n,e.observers&&e.observers.length&&I(()=>{for(let o=0;o<e.observers.length;o+=1){const i=e.observers[o],r=w&&w.running;r&&w.disposed.has(i),(r&&!i.tState||!r&&!i.state)&&(i.pure?h.push(i):v.push(i),i.observers&&ce(i)),r||(i.state=S)}if(h.length>1e6)throw h=[],new Error},!1)),n}function F(e){if(!e.fn)return;Q(e);const n=m,t=a,s=H;a=m=e,_e(e,e.value,s),a=t,m=n}function _e(e,n,t){let s;try{s=e.fn(n)}catch(o){e.pure&&(e.state=S),ue(o)}(!e.updatedAt||e.updatedAt<=t)&&(e.updatedAt!=null&&"observers"in e?oe(e,s):e.value=s,e.updatedAt=t)}function le(e,n,t,s=S,o){const i={fn:e,state:s,updatedAt:null,owned:null,sources:null,sourceSlots:null,cleanups:null,value:n,owner:m,context:null,pure:t};return m===null||m!==ne&&(m.owned?m.owned.push(i):m.owned=[i]),i}function ie(e){const n=w;if(e.state===0||n)return;if(e.state===q||n)return j(e);if(e.suspense&&J(e.suspense.inFallback))return e.suspense.effects.push(e);const t=[e];for(;(e=e.owner)&&(!e.updatedAt||e.updatedAt<H);)(e.state||n)&&t.push(e);for(let s=t.length-1;s>=0;s--)if(e=t[s],e.state===S||n)F(e);else if(e.state===q||n){const o=h;h=null,I(()=>j(e,t[0]),!1),h=o}}function I(e,n){if(h)return e();let t=!1;n||(h=[]),v?t=!0:v=[],H++;try{const s=e();return be(t),s}catch(s){h||(v=null),ue(s)}}function be(e){if(h&&(re(h),h=null),e)return;const n=v;v=null,n.length&&I(()=>ge(n),!1)}function re(e){for(let n=0;n<e.length;n++)ie(e[n])}function j(e,n){const t=w;e.state=0;for(let s=0;s<e.sources.length;s+=1){const o=e.sources[s];o.sources&&(o.state===S||t?o!==n&&ie(o):(o.state===q||t)&&j(o,n))}}function ce(e){const n=w;for(let t=0;t<e.observers.length;t+=1){const s=e.observers[t];(!s.state||n)&&(s.state=q,s.pure?h.push(s):v.push(s),s.observers&&ce(s))}}function Q(e){let n;if(e.sources)for(;e.sources.length;){const t=e.sources.pop(),s=e.sourceSlots.pop(),o=t.observers;if(o&&o.length){const i=o.pop(),r=t.observerSlots.pop();s<o.length&&(i.sourceSlots[r]=s,o[s]=i,t.observerSlots[s]=r)}}if(e.owned){for(n=0;n<e.owned.length;n++)Q(e.owned[n]);e.owned=null}if(e.cleanups){for(n=0;n<e.cleanups.length;n++)e.cleanups[n]();e.cleanups=null}e.state=0,e.context=null}function xe(e){return e instanceof Error||typeof e=="string"?e:new Error("Unknown error")}function ue(e){throw e=xe(e),e}function ye(e,n){return J(()=>e(n||{}))}function we(e,n,t){let s=t.length,o=n.length,i=s,r=0,l=0,u=n[o-1].nextSibling,f=null;for(;r<o||l<i;){if(n[r]===t[l]){r++,l++;continue}for(;n[o-1]===t[i-1];)o--,i--;if(o===r){const p=i<s?l?t[l-1].nextSibling:t[i-l]:u;for(;l<i;)e.insertBefore(t[l++],p)}else if(i===l)for(;r<o;)(!f||!f.has(n[r]))&&n[r].remove(),r++;else if(n[r]===t[i-1]&&t[l]===n[o-1]){const p=n[--o].nextSibling;e.insertBefore(t[l++],n[r++].nextSibling),e.insertBefore(t[--i],p),n[o]=t[i]}else{if(!f){f=new Map;let $=l;for(;$<i;)f.set(t[$],$++)}const p=f.get(n[r]);if(p!=null)if(l<p&&p<i){let $=r,x=1,P;for(;++$<o&&$<i&&!((P=f.get(n[$]))==null||P!==p+x);)x++;if(x>p-l){const O=n[r];for(;l<p;)e.insertBefore(t[l++],O)}else e.replaceChild(t[l++],n[r++])}else r++;else n[r++].remove()}}}const ee="_$DX_DELEGATE";function ve(e,n,t,s={}){let o;return me(i=>{o=i,n===document?e():_(n,e(),n.firstChild?null:void 0,t)},s.owner),()=>{o(),n.textContent=""}}function b(e,n,t){const s=document.createElement("template");s.innerHTML=e;let o=s.content.firstChild;return t&&(o=o.firstChild),o}function Se(e,n=window.document){const t=n[ee]||(n[ee]=new Set);for(let s=0,o=e.length;s<o;s++){const i=e[s];t.has(i)||(t.add(i),n.addEventListener(i,Ae))}}function Ne(e,n,t){t==null?e.removeAttribute(n):e.setAttribute(n,t)}function N(e,n){n==null?e.removeAttribute("class"):e.className=n}function _(e,n,t,s){if(t!==void 0&&!s&&(s=[]),typeof n!="function")return B(e,n,s,t);E(o=>B(e,n(),o,t),s)}function Ae(e){const n=`$$${e.type}`;let t=e.composedPath&&e.composedPath()[0]||e.target;for(e.target!==t&&Object.defineProperty(e,"target",{configurable:!0,value:t}),Object.defineProperty(e,"currentTarget",{configurable:!0,get(){return t||document}}),y.registry&&!y.done&&(y.done=!0,document.querySelectorAll("[id^=pl-]").forEach(s=>s.remove()));t!==null;){const s=t[n];if(s&&!t.disabled){const o=t[`${n}Data`];if(o!==void 0?s.call(t,o,e):s.call(t,e),e.cancelBubble)return}t=t.host&&t.host!==t&&t.host instanceof Node?t.host:t.parentNode}}function B(e,n,t,s,o){for(y.context&&!t&&(t=[...e.childNodes]);typeof t=="function";)t=t();if(n===t)return t;const i=typeof n,r=s!==void 0;if(e=r&&t[0]&&t[0].parentNode||e,i==="string"||i==="number"){if(y.context)return t;if(i==="number"&&(n=n.toString()),r){let l=t[0];l&&l.nodeType===3?l.data=n:l=document.createTextNode(n),t=A(e,t,s,l)}else t!==""&&typeof t=="string"?t=e.firstChild.data=n:t=e.textContent=n}else if(n==null||i==="boolean"){if(y.context)return t;t=A(e,t,s)}else{if(i==="function")return E(()=>{let l=n();for(;typeof l=="function";)l=l();t=B(e,l,t,s)}),()=>t;if(Array.isArray(n)){const l=[],u=t&&Array.isArray(t);if(G(l,n,t,o))return E(()=>t=B(e,l,t,s,!0)),()=>t;if(y.context){if(!l.length)return t;for(let f=0;f<l.length;f++)if(l[f].parentNode)return t=l}if(l.length===0){if(t=A(e,t,s),r)return t}else u?t.length===0?te(e,l,s):we(e,t,l):(t&&A(e),te(e,l));t=l}else if(n instanceof Node){if(y.context&&n.parentNode)return t=r?[n]:n;if(Array.isArray(t)){if(r)return t=A(e,t,s,n);A(e,t,null,n)}else t==null||t===""||!e.firstChild?e.appendChild(n):e.replaceChild(n,e.firstChild);t=n}}return t}function G(e,n,t,s){let o=!1;for(let i=0,r=n.length;i<r;i++){let l=n[i],u=t&&t[i];if(l instanceof Node)e.push(l);else if(!(l==null||l===!0||l===!1))if(Array.isArray(l))o=G(e,l,u)||o;else if(typeof l=="function")if(s){for(;typeof l=="function";)l=l();o=G(e,Array.isArray(l)?l:[l],Array.isArray(u)?u:[u])||o}else e.push(l),o=!0;else{const f=String(l);u&&u.nodeType===3&&u.data===f?e.push(u):e.push(document.createTextNode(f))}}return o}function te(e,n,t=null){for(let s=0,o=n.length;s<o;s++)e.insertBefore(n[s],t)}function A(e,n,t,s){if(t===void 0)return e.textContent="";const o=s||document.createTextNode("");if(n.length){let i=!1;for(let r=n.length-1;r>=0;r--){const l=n[r];if(o!==l){const u=l.parentNode===e;!i&&!r?u?e.replaceChild(o,l):e.insertBefore(o,t):u&&l.remove()}else i=!0}}else e.insertBefore(o,t);return[o]}const Ce="_App_705p1_1",Ee="_logo_705p1_5",Te="_header_705p1_11",Ie="_link_705p1_23",Pe="_pane_705p1_36",Oe="_row_705p1_46",C={App:Ce,logo:Ee,"logo-spin":"_logo-spin_705p1_1",header:Te,link:Ie,pane:Pe,row:Oe},k="",R={method:"POST",mode:"cors",cache:"no-cache",credentials:"same-origin",headers:{"Content-Type":"application/json"},redirect:"follow",referrerPolicy:"no-referrer",body:""},De=async e=>{const n=k+"/create/"+e;(await fetch(n,R)).json().then(()=>alert("Success!")).catch(()=>alert("Something went wrong"))},Le=async(e,n,t)=>{const s=`${k}/insert/${e}/${n}`;(await fetch(s,{...R,body:t})).json().then(()=>alert("Success!")).catch(()=>alert("Something went wrong"))},Ue=async(e,n)=>{const t=`${k}/insert_url/${e}`;(await fetch(t,{...R,body:n})).json().then(()=>alert("Success!")).catch(()=>alert("Something went wrong"))},qe=async(e,n,t)=>{const s=`${k}/search/${e}/${n}`,o=await fetch(s,{...R,body:void 0,method:"GET"});t(await o.json())},je=b("<a></a>"),Be=b("<div><h3>Documents Found</h3><ul></ul></div>"),Fe=b("<li></li>"),ke=b('<div><header><div><div><h1>Index</h1><input type="text" placeholder="index name"><br></div><div><h1>Add document</h1><textarea placeholder="document name or url"></textarea><br><textarea placeholder="document text"></textarea><br></div></div><div><h1>Search</h1><textarea cols="100" placeholder="Query"></textarea><br><br></div></header></div>'),Re=b("<button>Set Index</button>"),M=b("<p>Please select index</p>"),Me=b("<button>Index Document</button>"),Ge=b("<button>Index Url</button>"),He=b("<button>Search</button>"),[d,T]=$e({indexName:localStorage.getItem("indexName")||""}),Je=()=>{const e=()=>{const r=d();Le(r.indexName||"",r.documentName||"",r.documentText||"")},n=()=>{const r=d();Ue(r.indexName||"",r.documentName||"")},t=()=>{const r=d();qe(r.indexName||"",r.query||"",l=>{T({...d(),results:l.response})})},s=()=>De(d().indexName||""),o=r=>r.includes("http")?(()=>{const l=je.cloneNode(!0);return _(l,()=>JSON.parse(r).name),E(()=>Ne(l,"href",JSON.parse(r).url)),l})():r,i=()=>{if(!!d().results)return d().results?.length===0?"No results":(()=>{const r=Be.cloneNode(!0),l=r.firstChild,u=l.nextSibling;return _(u,()=>d().results?.map(f=>(()=>{const p=Fe.cloneNode(!0);return _(p,()=>o(f)),p})())),r})()};return(()=>{const r=ke.cloneNode(!0),l=r.firstChild,u=l.firstChild,f=u.firstChild,p=f.firstChild,$=p.nextSibling;$.nextSibling;const x=f.nextSibling,P=x.firstChild,O=P.nextSibling,fe=O.nextSibling,V=fe.nextSibling;V.nextSibling;const D=u.nextSibling,ae=D.firstChild,K=ae.nextSibling,de=K.nextSibling,he=de.nextSibling;return $.$$input=c=>{localStorage.setItem("indexName",c.target.value),T({...d(),indexName:c.target.value})},_(f,(()=>{const c=L(()=>!!d().indexName);return()=>c()?(()=>{const g=Re.cloneNode(!0);return g.$$click=s,g})():M.cloneNode(!0)})(),null),O.$$input=c=>T({...d(),documentName:c.target.value}),V.$$input=c=>T({...d(),documentText:c.target.value}),_(x,(()=>{const c=L(()=>!!d().indexName);return()=>c()?(()=>{const g=Me.cloneNode(!0);return g.$$click=e,g})():M.cloneNode(!0)})(),null),_(x,(()=>{const c=L(()=>!!d().indexName);return()=>c()?(()=>{const g=Ge.cloneNode(!0);return g.$$click=n,g})():[]})(),null),K.$$input=c=>T({...d(),query:c.target.value}),_(D,(()=>{const c=L(()=>!!d().indexName);return()=>c()?(()=>{const g=He.cloneNode(!0);return g.$$click=t,g})():M.cloneNode(!0)})(),he),_(D,i,null),E(c=>{const g=C.App,W=C.header,X=C.row,Y=C.pane,Z=C.pane,z=C.pane;return g!==c._v$&&N(r,c._v$=g),W!==c._v$2&&N(l,c._v$2=W),X!==c._v$3&&N(u,c._v$3=X),Y!==c._v$4&&N(f,c._v$4=Y),Z!==c._v$5&&N(x,c._v$5=Z),z!==c._v$6&&N(D,c._v$6=z),c},{_v$:void 0,_v$2:void 0,_v$3:void 0,_v$4:void 0,_v$5:void 0,_v$6:void 0}),E(()=>$.value=d().indexName),r})()};Se(["input","click"]);ve(()=>ye(Je,{}),document.getElementById("root"));
