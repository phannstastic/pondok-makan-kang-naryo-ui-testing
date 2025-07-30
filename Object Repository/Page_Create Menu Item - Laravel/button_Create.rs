<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>button_Create</name>
   <tag></tag>
   <elementGuidId>bc597630-1e18-49d8-8c3a-953f09c42633</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//button[@id='key-bindings-1']</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value>#key-bindings-1</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>button</value>
      <webElementGuid>e830b70a-2bcb-436a-a9d6-7c3a8eabd787</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>x-bind:id</name>
      <type>Main</type>
      <value>$id('key-bindings')</value>
      <webElementGuid>cdf13d21-aee2-428b-bfb3-5fdb7a549edc</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>x-mousetrap.global.mod-s</name>
      <type>Main</type>
      <value>document.getElementById($el.id).click()</value>
      <webElementGuid>8ea56f1e-e17e-4184-afe1-8bc3245303ab</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>x-data</name>
      <type>Main</type>
      <value>{
            form: null,
            isProcessing: false,
            processingMessage: null,
        }</value>
      <webElementGuid>5d1e8ead-87d4-47d6-a613-716f3dc90a9b</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>x-init</name>
      <type>Main</type>
      <value>
            form = $el.closest('form')

            form?.addEventListener('form-processing-started', (event) => {
                isProcessing = true
                processingMessage = event.detail.message
            })

            form?.addEventListener('form-processing-finished', () => {
                isProcessing = false
            })
        </value>
      <webElementGuid>d0e0197e-78b9-4117-9e91-0beb3028eecd</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>x-bind:class</name>
      <type>Main</type>
      <value>{ 'enabled:opacity-70 enabled:cursor-wait': isProcessing }</value>
      <webElementGuid>fb79b54a-3d72-4ad8-8519-6c7615c8fa07</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>fi-btn relative grid-flow-col items-center justify-center font-semibold outline-none transition duration-75 focus-visible:ring-2 rounded-lg fi-color-custom fi-btn-color-primary fi-color-primary fi-size-md fi-btn-size-md gap-1.5 px-3 py-2 text-sm inline-grid shadow-sm bg-custom-600 text-white hover:bg-custom-500 focus-visible:ring-custom-500/50 dark:bg-custom-500 dark:hover:bg-custom-400 dark:focus-visible:ring-custom-400/50 fi-ac-action fi-ac-btn-action</value>
      <webElementGuid>0f21f7d8-3122-491e-a2a7-14333b106772</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>type</name>
      <type>Main</type>
      <value>submit</value>
      <webElementGuid>8efab344-ecb6-48c7-ac68-23a9cada6f8d</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>wire:loading.attr</name>
      <type>Main</type>
      <value>disabled</value>
      <webElementGuid>a3082ecf-b74c-4f3d-9976-d28f1929d1f4</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>x-bind:disabled</name>
      <type>Main</type>
      <value>isProcessing</value>
      <webElementGuid>f7bae2cd-5c83-4398-81e7-33d8662b3b65</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>id</name>
      <type>Main</type>
      <value>key-bindings-1</value>
      <webElementGuid>aeecd334-3f1c-444a-a24a-f2e94cd2d18c</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
            

                    
    
    

        

                    
    
    

        
    

    
        Create
    

            
    

    

    
</value>
      <webElementGuid>c12070f2-f6e8-4076-a3d0-248f01c1290b</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;key-bindings-1&quot;)</value>
      <webElementGuid>23526182-57b4-4346-bdae-cf6978bbbd43</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:attributes</name>
      <type>Main</type>
      <value>//button[@id='key-bindings-1']</value>
      <webElementGuid>6b01a4fe-c280-4475-bfe0-779503f9e77c</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//form[@id='form']/div[2]/div/button</value>
      <webElementGuid>ff18a88b-777d-4e5d-b349-487a94f88126</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Gambar Menu'])[1]/following::button[1]</value>
      <webElementGuid>1df1f69a-580f-4182-8f16-9253715460ca</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Rp'])[1]/following::button[1]</value>
      <webElementGuid>6dbcd013-46bf-4179-bae2-cac73d27cc67</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Create &amp; create another'])[1]/preceding::button[1]</value>
      <webElementGuid>c70d9eb8-87e0-4318-9a67-b96721de4277</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//form/div[2]/div/button</value>
      <webElementGuid>1dfc6057-0b83-4722-b976-6b9894c3b427</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//button[@type = 'submit' and @id = 'key-bindings-1' and (text() = '
            

                    
    
    

        

                    
    
    

        
    

    
        Create
    

            
    

    

    
' or . = '
            

                    
    
    

        

                    
    
    

        
    

    
        Create
    

            
    

    

    
')]</value>
      <webElementGuid>9b8c4886-5c28-4a03-9849-48926a30384c</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
