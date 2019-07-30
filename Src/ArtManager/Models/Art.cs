// Copyright (c) Anthony Wilcox and contributors. All rights reserved.
// Licensed under the GNU GPL v3 license. See LICENSE file in the project
// root for full license information.
using System;
using System.Diagnostics;
using System.IO;
using System.Threading.Tasks;
using LiteDB;
using Newtonsoft.Json;

namespace ArtManager.Models
{
    public enum Catagory
    {
        Unknown,
        Request,
        Commission,
        YCH,
        Raffle,
    }
    public class Customer
    {
        public string Name { get; set; }
        public string Contact { get; set; }
        public string Payment { get; set; }
    }

    public class Art
    {

        public string Hash
        {
            get
            {
                return ArtUtils.CalculateHash(this);
            }
        }
        public DateTime Timestamp { get; internal set; } = DateTime.Now;

        // Only used in data export
        [BsonIgnore]
        public string Version { get; } = "0.2";

        // Ignored in Json export since 0.1.1
        [JsonIgnore]
        public Catagory Catagory
        {
            get
            {
                if (Slot.HasValue && Ticket.HasValue)
                    return Catagory.YCH;
                if (Description != string.Empty && Price.HasValue)
                    return Catagory.Commission;
                else
                    return Catagory.Request;
            }
        }

        public string Name { get; set; }
        public int? Ticket { get; set; }
        public int? Slot { get; set; }
        public decimal? Price { get; set; }
        public string Reference { get; set; }
        public string Description { get; set; }
        public Customer Custmer { get; set; }

        public void Save(string dir, string filename)
        {
            var json = ArtUtils.AsJson(this);
            var path = Path.Combine(dir, $"{filename}.artm");

            try
            {
                using (var writer = new StreamWriter(path))
                {
                    writer.Write(json);
                }

                if (Debugger.IsAttached)
                    Debug.WriteLine(json);
            }
            catch (IOException err)
            {
                throw new IOException(err.Message);
            }
            catch (Exception err)
            {
                throw new Exception(err.Message);
            }
        }

        /// <summary>
        /// Writes a formatted Json file with the proper data provided
        /// by the database. Exention is not required.
        /// </summary>
        /// <param name="dir">Path to directory</param>
        /// <param name="filename">[filename].artm</param>
        /// <returns></returns>
        public async Task SaveAsync(string dir, string filename)
        {
            var json = ArtUtils.AsJson(this);
            var path = Path.Combine(dir, $"{filename}.artm");

            try
            {
                using (var writer = new StreamWriter(path))
                {
                    await writer.WriteAsync(json);
                }

                if (Debugger.IsAttached)
                    Debug.WriteLine(json);
            }
            catch (IOException err)
            {
                throw new IOException(err.Message);
            }
            catch (Exception err)
            {
                throw new Exception(err.Message);
            }
        }
    }
}
