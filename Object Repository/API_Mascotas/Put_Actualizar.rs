<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Put_Actualizar</name>
   <tag></tag>
   <elementGuidId>48a6502b-467f-4d46-b387-83134d55b261</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>true</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;id\&quot;: 1030671099,\n  \&quot;name\&quot;: \&quot;${Nombre}\&quot;,\n  \&quot;category\&quot;: {\n    \&quot;id\&quot;: 0,\n    \&quot;name\&quot;: \&quot;${Raza}\&quot;\n  },\n  \&quot;photoUrls\&quot;: [\n    \&quot;${Foto}\&quot;\n  ],\n  \&quot;tags\&quot;: [\n    {\n      \&quot;id\&quot;: 0,\n      \&quot;name\&quot;: \&quot;${Categoria}\&quot;\n    }\n  ],\n  \&quot;status\&quot;: \&quot;${Estado}\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>069cc1b1-8470-4637-849a-2ea0ef3e7121</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.2.3</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>https://petstore.swagger.io/v2/pet</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'Sofka2'</defaultValue>
      <description></description>
      <id>b2cad1cd-331f-4c44-99c6-3cec11ea1f04</id>
      <masked>false</masked>
      <name>Nombre</name>
   </variables>
   <variables>
      <defaultValue>'gato'</defaultValue>
      <description></description>
      <id>2bd9aef3-f92c-4cbd-8760-3dede045d51f</id>
      <masked>false</masked>
      <name>Raza</name>
   </variables>
   <variables>
      <defaultValue>'UrlFoto'</defaultValue>
      <description></description>
      <id>84de0cad-b68b-4f7c-8949-8f24d8f37d4c</id>
      <masked>false</masked>
      <name>Foto</name>
   </variables>
   <variables>
      <defaultValue>'Rabioso'</defaultValue>
      <description></description>
      <id>d15c41c0-0c50-4322-b2e0-335135de98d1</id>
      <masked>false</masked>
      <name>Categoria</name>
   </variables>
   <variables>
      <defaultValue>'No Disponible'</defaultValue>
      <description></description>
      <id>b081758f-b8b5-49a6-96ca-f5d95312622b</id>
      <masked>false</masked>
      <name>Estado</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
