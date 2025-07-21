<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>post_AddPet</name>
   <tag></tag>
   <elementGuidId>205cd3fa-e296-4b71-8c3c-692de4a2e137</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>true</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;\n{\n  \&quot;id\&quot;: 1030671099,\n  \&quot;name\&quot;: \&quot;${Nombre}\&quot;,\n  \&quot;category\&quot;: {\n    \&quot;id\&quot;: 0,\n    \&quot;name\&quot;: \&quot;${Raza}\&quot;\n  },\n  \&quot;photoUrls\&quot;: [\n    \&quot;${Foto}\&quot;\n  ],\n  \&quot;tags\&quot;: [\n    {\n      \&quot;id\&quot;: 0,\n      \&quot;name\&quot;: \&quot;${Categoria}\&quot;\n    }\n  ],\n  \&quot;status\&quot;: \&quot;${Estado}\&quot;\n}&quot;,
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
      <webElementGuid>d43838c8-763d-460d-9032-61e139df145b</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.2.3</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
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
      <defaultValue>'Firulais'</defaultValue>
      <description></description>
      <id>56aa6b6d-94a6-494b-99ea-842a7bec26ba</id>
      <masked>false</masked>
      <name>Nombre</name>
   </variables>
   <variables>
      <defaultValue>'Perro'</defaultValue>
      <description></description>
      <id>cea75b01-5566-4255-b9d1-98fe5f3e8f4e</id>
      <masked>false</masked>
      <name>Raza</name>
   </variables>
   <variables>
      <defaultValue>'Amistoso'</defaultValue>
      <description></description>
      <id>bc3ab075-31ff-43b5-9528-a578d68216a6</id>
      <masked>false</masked>
      <name>Categoria</name>
   </variables>
   <variables>
      <defaultValue>'Disponible'</defaultValue>
      <description></description>
      <id>7ac61c3f-281b-47c5-916b-df0b758ac262</id>
      <masked>false</masked>
      <name>Estado</name>
   </variables>
   <variables>
      <defaultValue>'url'</defaultValue>
      <description></description>
      <id>a2405c7b-9202-4010-9a96-be2f7011b48e</id>
      <masked>false</masked>
      <name>Foto</name>
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
